```
git clone https://github.com/pwoolcoc/moved-followers
cd moved-followers
cargo run -- "mastodon.social"                # to just list the moved accounts
cargo run -- "mastodon.social" --follow-new   # to list the accounts and follow the new ones
cargo run -- "mastodon.social" --unfollow-old # to list the accounts and unfollow the old ones
cargo run -- "mastodon.social" --follow-new --unfollow-old
```

