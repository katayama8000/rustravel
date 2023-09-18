struct MyStruct {
    data: i32,
}

impl MyStruct {
    // 不変のself参照を受け取るメソッド
    fn get_data(&self) -> i32 {
        self.data
    }

    // 可変のself参照を受け取るメソッド
    fn set_data(&mut self, new_data: i32) {
        self.data = new_data;
    }

    // 所有権を奪うメソッド
    fn consume(self) -> i32 {
        self.data
    }
}

fn main() {
    let mut obj = MyStruct { data: 42 };

    // 不変のself参照を使用
    let value = obj.get_data();

    // 可変のself参照を使用
    obj.set_data(100);

    // 所有権を奪うメソッドを使用
    let owned_value = obj.consume();
}
