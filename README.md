# [zk + pinocchio]のhelloworld

## 完了事項
- molluskのセットアップ
- 回路のセットアップ
- offchainでは、verifyは成功するということ
- onchainでのnegとendian変更をoffchainに移植
- verify keyの取り出し関数の実装
- endian変更関数

## 残タスク
- instruction_dataを渡す
- verify keyのendianの変更（確認）
- prrof_aなどのcompressed or uncomporessedのチェック
- テストケース追加（happy path以外）

## 完成形（予定）
- 現在の回路がonchainでverifyされること
- そのときにかかるcuの数値を出すこと
- 回路の変更のみでpinocchioで使えるzkの枠組みの完成

