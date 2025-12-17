use anchor_lang::prelude::*;

cfg_if::cfg_if! {
    if #[cfg(feature = "mainnet-beta")] {
        declare_id!("4sbjN16fgtnGMFaJ6s7aw9ha1uUQ9qLBv5gHSui2aNUU");
    } else if #[cfg(feature = "devnet")] {
        declare_id!("4sbjN16fgtnGMFaJ6s7aw9ha1uUQ9qLBv5gHSui2aNUU");
    } else if #[cfg(feature = "staging")] {
        declare_id!("stag8sTKds2h4KzjUw3zKTsxbqvT4XKHdaR9X9E6Rct");
    } else {
        declare_id!("2jGhuVUuy3umdzByFx8sNWUAaf5vaeuDm78RDPEnhrMr");
    }
}
