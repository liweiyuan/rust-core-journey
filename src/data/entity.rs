use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    pub x: i32,
    pub y: i32,
}
#[derive(Deserialize, Serialize, Debug)]
struct Data {
    name: String,
    age: u32,
    points: Vec<Point>,
    is_active: bool,
    optional_value: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_serialization() {
        let point = Point { x: 10, y: 20 };

        // 测试序列化
        let serialized = serde_json::to_string(&point).unwrap();
        assert_eq!(serialized, r#"{"x":10,"y":20}"#);

        // 测试反序列化
        let deserialized: Point = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.x, 10);
        assert_eq!(deserialized.y, 20);
    }

    #[test]
    fn test_data_serialization() {
        let data = Data {
            name: "张三".to_string(),
            age: 25,
            points: vec![Point { x: 1, y: 2 }, Point { x: 3, y: 4 }],
            is_active: true,
            optional_value: Some("测试值".to_string()),
        };

        // 测试序列化
        let serialized = serde_json::to_string(&data).unwrap();
        let expected = r#"{"name":"张三","age":25,"points":[{"x":1,"y":2},{"x":3,"y":4}],"is_active":true,"optional_value":"测试值"}"#;
        assert_eq!(serialized, expected);

        // 测试反序列化
        let deserialized: Data = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.name, "张三");
        assert_eq!(deserialized.age, 25);
        assert_eq!(deserialized.points.len(), 2);
        assert_eq!(deserialized.points[0].x, 1);
        assert_eq!(deserialized.points[0].y, 2);
        assert_eq!(deserialized.points[1].x, 3);
        assert_eq!(deserialized.points[1].y, 4);
        assert!(deserialized.is_active);
        assert_eq!(deserialized.optional_value, Some("测试值".to_string()));
    }

    #[test]
    fn test_data_with_none_optional() {
        let data = Data {
            name: "李四".to_string(),
            age: 30,
            points: vec![Point { x: 5, y: 6 }],
            is_active: false,
            optional_value: None,
        };

        // 测试序列化
        let serialized = serde_json::to_string(&data).unwrap();
        let expected = r#"{"name":"李四","age":30,"points":[{"x":5,"y":6}],"is_active":false,"optional_value":null}"#;
        assert_eq!(serialized, expected);

        // 测试反序列化
        let deserialized: Data = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.name, "李四");
        assert_eq!(deserialized.age, 30);
        assert_eq!(deserialized.points.len(), 1);
        assert_eq!(deserialized.points[0].x, 5);
        assert_eq!(deserialized.points[0].y, 6);
        assert!(!deserialized.is_active);
        assert_eq!(deserialized.optional_value, None);
    }

    #[test]
    fn test_invalid_json_deserialization() {
        // 测试无效的 JSON 字符串
        let invalid_json = r#"{"x": "not_a_number", "y": 20}"#;
        let result = serde_json::from_str::<Point>(invalid_json);
        assert!(result.is_err());
    }
}
