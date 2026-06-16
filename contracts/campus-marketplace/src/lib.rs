#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env};

// 1. Định nghĩa cấu trúc dữ liệu cho một Sản phẩm (Item)
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Product {
    pub id: u32,
    pub seller: Address,
    pub buyer: Option<Address>,
    pub price: i128,
    pub is_sold: bool,
    pub is_confirmed: bool,
}

// Định nghĩa các Key để lưu trữ dữ liệu vào Blockchain State
#[contracttype]
pub enum DataKey {
    Product(u32),       // Lưu chi tiết từng sản phẩm theo ID (Persistent Storage)
    ProductCount,       // Lưu tổng số lượng sản phẩm (Instance Storage)
    CampusTokenAddress, // Lưu địa chỉ của đồng token CAMPUS dùng để thanh toán
}

#[contract]
pub struct CampusMarketplace;

#[contractimpl]
impl CampusMarketplace {
    
    // Khởi tạo Contract với địa chỉ của đồng Token CAMPUS
    pub fn initialize(env: Env, token_address: Address) {
        if env.storage().instance().has(&DataKey::CampusTokenAddress) {
            panic!("Contract da duoc khoi tao tu truoc!");
        }
        env.storage().instance().set(&DataKey::CampusTokenAddress, &token_address);
        env.storage().instance().set(&DataKey::ProductCount, &0u32);
    }

    // 2. Tính năng: Sinh viên Đăng bán sản phẩm (Đặt giá bằng CAMPUS token)
    pub fn list_product(env: Env, seller: Address, price: i128) -> u32 {
        // Xác thực chữ ký của người bán để đảm bảo chính chủ đăng
        seller.require_auth();

        if price <= 0 {
            panic!("Gia san pham phai lon hon 0");
        }

        // Lấy ID tự động tăng
        let mut count: u32 = env.storage().instance().get(&DataKey::ProductCount).unwrap_or(0);
        count += 1;

        let new_product = Product {
            id: count,
            seller: seller.clone(),
            buyer: None,
            price,
            is_sold: false,
            is_confirmed: false,
        };

        // Lưu sản phẩm vào kho lưu trữ dài hạn (Persistent Storage)
        env.storage().persistent().set(&DataKey::Product(count), &new_product);
        // Cập nhật bộ đếm trong Instance Storage
        env.storage().instance().set(&DataKey::ProductCount, &count);

        count // Trả về ID của sản phẩm vừa đăng
    }

    // 3. Tính năng: Mua hàng & Đặt cọc (Escrow)
    // Tiền từ người mua sẽ bị khóa lại trong Smart Contract này chứ chưa chuyển ngay cho người bán
    pub fn buy_and_lock_funds(env: Env, buyer: Address, product_id: u32) {
        buyer.require_auth();

        // Lấy thông tin sản phẩm từ Storage
        let product_key = DataKey::Product(product_id);
        if !env.storage().persistent().has(&product_key) {
            panic!("San pham khong ton tai");
        }
        let mut product: Product = env.storage().persistent().get(&product_key).unwrap();

        if product.is_sold {
            panic!("San pham nay da co nguoi mua");
        }

        // Người bán không thể tự mua sản phẩm của chính mình
        if buyer == product.seller {
            panic!("Nguoi ban khong the mua san pham cua chinh minh");
        }

        // Lấy token CAMPUS để xử lý giao dịch tiền tệ
        let token_address: Address = env.storage().instance().get(&DataKey::CampusTokenAddress).unwrap();
        let client = token::Client::new(&env, &token_address);

        // Chuyển tiền từ ví Người mua (Buyer) VÀO ví của chính Smart Contract này (Giữ tiền hộ)
        client.transfer(&buyer, &env.current_contract_address(), &product.price);

        // Cập nhật trạng thái sản phẩm
        product.buyer = Some(buyer);
        product.is_sold = true;

        env.storage().persistent().set(&product_key, &product);
    }

    // 4. Tính năng: Xác nhận đã nhận hàng & Giải ngân tiền cho Người bán
    pub fn confirm_delivery(env: Env, buyer: Address, product_id: u32) {
        buyer.require_auth();

        let product_key = DataKey::Product(product_id);
        if !env.storage().persistent().has(&product_key) {
            panic!("San pham khong ton tai");
        }
        let mut product: Product = env.storage().persistent().get(&product_key).unwrap();

        // Chỉ có người mua của sản phẩm này mới có quyền xác nhận
        if product.buyer != Some(buyer) {
            panic!("Ban khong phai la nguoi mua san pham nay");
        }

        if product.is_confirmed {
            panic!("Don hang nay da duoc hoan thanh truoc do");
        }

        // CHUẨN BẢO MẬT: Cập nhật trạng thái hoàn thành TRƯỚC KHI chuyển tiền
        product.is_confirmed = true;
        env.storage().persistent().set(&product_key, &product);

        // Giải ngân: Chuyển tiền đang bị khóa từ Smart Contract sang cho Người bán (Seller)
        let token_address: Address = env.storage().instance().get(&DataKey::CampusTokenAddress).unwrap();
        let client = token::Client::new(&env, &token_address);
        
        client.transfer(&env.current_contract_address(), &product.seller, &product.price);
    }

    // Hàm phụ trợ: Xem thông tin sản phẩm
    pub fn get_product(env: Env, product_id: u32) -> Product {
        env.storage().persistent().get(&DataKey::Product(product_id)).expect("San pham khong ton tai")
    }
}