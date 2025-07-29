mod cpp_import;

fn main() {
    println!("Hello, world!");
}

#[macro_export]
macro_rules! import_bp {
    () => {
        $crate::import_bp!("generated.rs");
    };
    ($($path:tt)*) => {
        include!(concat!(env!("OUT_DIR"), "/bprust-build-result/", $($path)*))
    }
}

macro_rules! BasicType {
    ($($(#[$attrs:meta])* $Type:ident $size:literal $align:literal;)*) => {
        $(
            $(#[$attrs])*
            #[repr(align($align))]
            pub struct $Type([u8; $size]);
        )*
    };
}

BasicType! {
    FName 12 4;
    FString 16 8;
    FText 16 8;
    FScriptArray 16 8;
    FScriptSet 80 8;
    FScriptMap 80 8;
    FSoftObjectPtr 48 8;
}
