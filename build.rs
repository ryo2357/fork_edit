// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

// このファイルはCargoビルド時に実行されるビルドスクリプトです。
// Windows環境でビルドする際に、実行ファイルにマニフェストやリソース情報を埋め込む処理を行います。

fn main() {
    #[cfg(windows)]
    // Windowsターゲットの場合のみ、リソース情報を設定
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default() == "windows" {
        winres::WindowsResource::new()
            .set_manifest_file("src/bin/edit/edit.exe.manifest") // 実行ファイルに埋め込むマニフェストファイルを指定
            .set("FileDescription", "Microsoft Edit")            // ファイルの説明を設定
            .set("LegalCopyright", "© Microsoft Corporation. All rights reserved.") // 著作権情報を設定
            .compile()
            .unwrap();
    }
}
