use crate::errors::ErrorCode;
use anchor_lang::prelude::*;
use pyth_min::price_update::PriceUpdateV2;

#[derive(Accounts)]
pub struct EchoPrice<'info> {
    /// CHECK: Validate owner and discriminator as needed!
    // #[account(owner = Pubkey::from(PYTH_FEED_OWNER))]
    pub price_acc: UncheckedAccount<'info>,
}

pub fn echo_price(ctx: Context<EchoPrice>) -> Result<()> {
    let data = &ctx.accounts.price_acc.try_borrow_data()?[..];

    // You should probably validate the discriminator...
    // let account_discriminator = &data[..8];
    // if account_discriminator != &[0x22, 0xF1, 0x23, 0x63, 0x9D, 0x7E, 0xF4, 0xCD] {
    //     panic!("bad discriminator")
    // }

    let price_v2 = PriceUpdateV2::get_price_update_v2_from_bytes(&data[8..]);

    let price = price_v2
        .get_price_no_older_than(
            Clock::get()?.unix_timestamp,
            100,  // in seconds
            None, // Pass the feed ID in bytes if you want to validate it
        )
        .map_err(|e| ErrorCode::from(e))?;

    msg!(
        "Price: {:?}, confidence: {:?}, publish time: {:?}",
        price.price,
        price.conf,
        price.publish_time
    );

    Ok(())
}
