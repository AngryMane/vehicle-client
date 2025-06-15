# Vehicle Shadow Client Library

Vehicle Signal Shadowサービスのための高レベルクライアントライブラリです。gRPCクライアントの接続管理、JSON解析、値フォーマットなどの共通機能を提供します。

## 機能

- **高レベルクライアント**: 簡単に使用できるVehicleShadowClient
- **JSON解析**: JSON文字列からState/Valueオブジェクトへの変換
- **値フォーマット**: Valueオブジェクトの人間が読みやすい文字列への変換
- **エラーハンドリング**: 統一されたエラー型と適切なエラー変換
- **型安全性**: Rustの型システムを活用した安全なAPI

## 使用方法

### 基本的な使用例

```rust
use vehicle_shadow_client::{VehicleShadowClient, format_signal};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // サーバーに接続
    let mut client = VehicleShadowClient::connect("http://localhost:50051").await?;
    
    // シグナルを取得
    let response = client.get_signals(vec!["Vehicle.Speed".to_string()]).await?;
    
    if response.success {
        for signal in response.signals {
            println!("{}", format_signal(&signal));
        }
    }
    
    // シグナル値を設定
    let response = client.set_signal(
        "Vehicle.Speed".to_string(),
        "60.5"
    ).await?;
    
    // 複数のシグナルを設定
    let signals = vec![
        ("Vehicle.Speed".to_string(), "60.5".to_string()),
        ("Vehicle.Engine.RPM".to_string(), "2000".to_string()),
    ];
    let response = client.set_signals(signals).await?;
    
    // シグナルの購読
    let mut stream = client.subscribe(vec!["Vehicle.Speed".to_string()]).await?;
    
    while let Some(response) = stream.message().await? {
        if let Some(signal) = response.signal {
            println!("Update: {}", signal.path);
        }
    }
    
    Ok(())
}
```

### JSON解析

```rust
use vehicle_shadow_client::{parse_state_from_json, parse_value_from_json};

// 単純な値の解析
let value = parse_value_from_json("42")?;
let value = parse_value_from_json("\"hello\"")?;
let value = parse_value_from_json("true")?;
let value = parse_value_from_json("[1, 2, 3]")?;

// Stateオブジェクトの解析
let state = parse_state_from_json("{\"value\": 60.5, \"capability\": true}")?;
let state = parse_state_from_json("60.5")?; // 単純な値もStateとして解析可能
```

### 値フォーマット

```rust
use vehicle_shadow_client::{format_value, format_signal};

// Valueオブジェクトのフォーマット
let formatted = format_value(&value);
println!("Value: {}", formatted);

// Signalオブジェクトのフォーマット
let formatted = format_signal(&signal);
println!("Signal:\n{}", formatted);
```

## API リファレンス

### VehicleShadowClient

#### メソッド

- `connect(server_url: &str) -> Result<Self>`: サーバーに接続してクライアントを作成
- `get_signals(paths: Vec<String>) -> Result<GetResponse>`: 指定されたパスのシグナルを取得
- `set_signal(path: String, value_json: &str) -> Result<SetResponse>`: 単一のシグナル値を設定
- `set_signals(signals: Vec<(String, String)>) -> Result<SetResponse>`: 複数のシグナル値を設定
- `subscribe(paths: Vec<String>) -> Result<Streaming<SubscribeResponse>>`: シグナル変更を購読
- `unsubscribe(paths: Vec<String>) -> Result<UnsubscribeResponse>`: 購読を解除
- `get_client() -> &mut SignalServiceClient<Channel>`: 低レベルクライアントへのアクセス

### パーサー関数

- `parse_state_from_json(json_str: &str) -> Result<State>`: JSON文字列をStateオブジェクトに解析
- `parse_value_from_json(json_str: &str) -> Result<Value>`: JSON文字列をValueオブジェクトに解析

### フォーマッター関数

- `format_value(value: &Value) -> String`: Valueオブジェクトを文字列にフォーマット
- `format_signal(signal: &Signal) -> String`: Signalオブジェクトを文字列にフォーマット

## エラーハンドリング

ライブラリは統一されたエラー型`ClientError`を提供します：

```rust
use vehicle_shadow_client::{ClientError, Result};

match client.get_signals(paths).await {
    Ok(response) => {
        // 成功時の処理
    }
    Err(ClientError::Transport(e)) => {
        // トランスポートエラー
    }
    Err(ClientError::JsonParse(e)) => {
        // JSON解析エラー
    }
    Err(ClientError::Service(e)) => {
        // サービスエラー
    }
    // その他のエラー...
}
```

## 依存関係

```toml
[dependencies]
vehicle-shadow-client = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```

## ライセンス

MITライセンスの下で公開されています。 