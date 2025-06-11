# oxc-transform-solid

:::warning
this project is drafted and implementing with GitHub Copilot. this is kinda toy project, and there is no enough test.
:::
高性能な Rust ベースの Solid.js JSX トランスフォーマー

## 概要

このプロジェクトは、[Solid.js](https://www.solidjs.com/) の JSX 変換を [OXC (Oxidation Compiler)](https://oxc.rs/) を使用して実装することで、従来の Babel ベースのトランスパイラーよりも大幅な性能向上を実現します。

**重要**: このプロジェクトは Solid.js のライブラリ（Signal、Effect など）の再実装は行いません。JSX の変換のみに焦点を当てています。

## 目標

- **高速化**: Babel プラグインと比較して 5-10倍の変換速度向上
- **メモリ効率**: 30-50% のメモリ使用量削減
- **完全互換**: 既存の Babel プラグインとの 100% 互換性
- **最適化**: コンパイル時の静的解析による追加最適化

## 機能

### 実装予定の変換機能

- **JSX エレメント変換**: `<div>content</div>` → `_tmpl$('<div>content</div>')`
- **コンポーネント変換**: カスタムコンポーネントの適切な関数呼び出しへの変換
- **プロパティバインディング**: 動的プロパティとイベントハンドラーの最適化
- **条件レンダリング**: `<Show>` や `<For>` などの制御フローコンポーネント
- **フラグメント処理**: React Fragment スタイルの構文サポート

### 最適化機能

- **静的解析**: コンパイル時での不要な反応性ラッパーの除去
- **テンプレート最適化**: 静的要素の事前コンパイル
- **TreeShaking 支援**: 未使用の Solid.js ユーティリティの識別

## アーキテクチャ

```
src/
├── lib.rs              # メインエントリポイント
├── transformer/
│   ├── jsx.rs          # JSX 変換ロジック
│   ├── components.rs   # コンポーネント変換
│   ├── events.rs       # イベントハンドラー最適化
│   └── optimization.rs # 静的解析と最適化
├── utils/
│   ├── ast_utils.rs    # AST 操作ユーティリティ
│   └── template.rs     # テンプレート生成
└── tests/
    ├── fixtures/       # テストケース
    └── integration/    # 統合テスト
```

## 開発状況

- [ ] プロジェクト基盤構築
- [ ] 基本的な JSX 変換エンジン
- [ ] コンポーネント変換
- [ ] イベントハンドラー最適化
- [ ] 制御フローコンポーネント
- [ ] 静的解析と最適化
- [ ] テストスイート
- [ ] ベンチマーク
- [ ] ドキュメント

## 使用方法

```rust
use oxc_transform_solid::SolidTransform;

// OXC トランスフォーマーとして使用
let transformer = SolidTransform::new();
// AST 変換実行
```

## ベンチマーク

| 変換器 | 速度 | メモリ使用量 | 出力サイズ |
|--------|------|-------------|-----------|
| Babel Plugin | 1x | 100% | 100% |
| OXC Transform | **8x** | **60%** | **85%** |

*注: ベンチマーク結果は予測値です。実際の結果は実装完了後に更新されます。*

## 貢献

1. リポジトリをフォーク
2. フィーチャーブランチを作成 (`git checkout -b feature/amazing-feature`)
3. 変更をコミット (`git commit -m 'Add amazing feature'`)
4. ブランチにプッシュ (`git push origin feature/amazing-feature`)
5. プルリクエストを開く

## ライセンス

MIT License - 詳細は [LICENSE](LICENSE) ファイルを参照してください。

## 関連プロジェクト

- [Solid.js](https://github.com/solidjs/solid) - 元のライブラリ
- [OXC](https://oxc.rs/) - Rust ベースの JavaScript ツールチェーン
- [babel-preset-solid](https://github.com/solidjs/solid/tree/main/packages/babel-preset-solid) - 元の Babel プラグイン