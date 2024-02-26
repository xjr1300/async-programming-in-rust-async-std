fn main() {
    async_std::task::block_on(async {
        panic!("test");
    });
}
