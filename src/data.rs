use crate::model::Product;
use crate::configuration::Settings;

pub fn fetch_products(_settings: &Settings) -> Vec<Product> {
    vec![
        Product {
            id: 1,
            name: "Logitech Wireless Mouse".to_string(),
            price: 29.99,
            description: "Navigate with precision using the Logitech Wireless Mouse. Features ergonomic design and seamless connectivity for enhanced productivity.".to_string(),
            image: "/catnip.jpg".to_string()
        },
        Product {
            id: 2,
            name: "Samsung 4K UHD Smart TV".to_string(),
            price: 499.99,
            description: "Upgrade your entertainment with the Samsung 4K UHD Smart TV. Experience vivid colors, sharp detail, and streaming capabilities.".to_string(),
            image: "/squid.jpg".to_string()
        },
        Product {
            id: 3,
            name: "Sony Noise-Canceling Headphones".to_string(),
            price: 249.99,
            description: "Immerse yourself in sound with Sony Noise-Canceling Headphones. Designed for crystal-clear audio and exceptional comfort.".to_string(),
            image: "/mermaid.jpg".to_string()
        },
        Product {
            id: 4,
            name: "Apple MacBook Air".to_string(),
            price: 999.99,
            description: "Power through your tasks with the Apple MacBook Air. Equipped with the M1 chip for blazing-fast performance and all-day battery life.".to_string(),
            image: "/ocean.jpg".to_string()
        },
        Product {
            id: 5,
            name: "Fitbit Charge 5".to_string(),
            price: 149.99,
            description: "Stay on top of your health with the Fitbit Charge 5. Tracks your fitness activities, sleep, and heart rate for a healthier lifestyle.".to_string(),
            image: "/pirate.jpg".to_string()
        },
        Product {
            id: 6,
            name: "Microsoft Surface Pro 8".to_string(),
            price: 1099.99,
            description: "Transform your workflow with the Microsoft Surface Pro 8. A versatile 2-in-1 laptop and tablet with a stunning touchscreen display.".to_string(),
            image: "/tug.jpg".to_string()
        },
        Product {
            id: 7,
            name: "Dyson V11 Vacuum Cleaner".to_string(),
            price: 699.99,
            description: "Keep your home spotless with the Dyson V11 Vacuum Cleaner. Engineered for powerful suction and intelligent cleaning modes.".to_string(),
            image: "/bed.jpg".to_string()
        },
        Product {
            id: 8,
            name: "Canon EOS Rebel T7 Camera".to_string(),
            price: 449.99,
            description: "Capture stunning photos and videos with the Canon EOS Rebel T7. Features a high-resolution sensor and easy-to-use interface.".to_string(),
            image: "/knot.jpg".to_string()
        },
        Product {
            id: 9,
            name: "Bose SoundLink Bluetooth Speaker".to_string(),
            price: 129.99,
            description: "Enjoy high-quality sound anywhere with the Bose SoundLink Bluetooth Speaker. Compact, portable, and equipped with a long-lasting battery.".to_string(),
            image: "/crabby.jpg".to_string()
        },
        Product {
            id: 10,
            name: "KitchenAid Stand Mixer".to_string(),
            price: 379.99,
            description: "Elevate your cooking and baking with the KitchenAid Stand Mixer. A versatile tool for kneading, mixing, and whipping ingredients.".to_string(),
            image: "/lifejacket.jpg".to_string()
        }
    ]
}
