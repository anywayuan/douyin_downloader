# 抖音视频下载工具

一个用 Rust 编写的抖音视频解析下载工具，支持自动提取分享链接并下载视频。

## 功能特点

✅ 自动提取抖音分享文本中的链接  
✅ **支持多种链接格式**（包括带连字符的新格式）🔗  
✅ 解析短链接获取真实视频地址  
✅ **多清晰度选择** (360P/540P/720P/1080P) 🎬  
✅ **优先下载无水印版本** 🎯  
✅ 下载视频并显示进度  
✅ 循环下载，支持连续下载多个视频  
✅ 多种获取方式，提高成功率  
✅ **跨平台支持** (Windows/macOS/Linux)

## 快速开始

### 方式一：下载预编译版本（推荐）⭐

**无需安装 Rust 环境，直接使用！**

1. 访问 [Releases 页面](../../releases)
2. 下载对应系统的文件：
   - **Windows**: `douyin_downloader-windows-x64.exe`
   - **macOS Intel**: `douyin_downloader-macos-x64`
   - **macOS Apple Silicon (M1/M2)**: `douyin_downloader-macos-arm64`
   - **Linux**: `douyin_downloader-linux-x64`

3. 双击运行（或在终端中运行）

#### macOS 用户注意事项 🍎

首次运行时，macOS 会显示安全警告：

```
未打开 "douyin_downloader"

Apple 无法验证 "douyin_downloader" 是否包含可能危害 Mac 安全或泄漏隐私的恶意软件。
```

**解决方法**：

**方法1：通过系统设置打开（推荐）**

1. 双击程序后出现警告，点击「完成」或「取消」
2. 打开「系统设置」→「隐私与安全性」
3. 滚动到底部，找到被阻止的程序提示
4. 点击「仍要打开」按钮
5. 在弹出的确认对话框中点击「打开」

**方法2：使用终端命令**

```bash
# 移除隔离属性
xattr -d com.apple.quarantine douyin_downloader-macos-x64

# 添加执行权限
chmod +x douyin_downloader-macos-x64

# 运行程序
./douyin_downloader-macos-x64
```

**方法3：右键打开**

1. 按住 `Control` 键，点击程序图标
2. 选择「打开」
3. 在弹出的对话框中点击「打开」

> 💡 **为什么会出现这个警告？**  
> 因为程序没有经过 Apple 的公证（需要付费开发者账号）。这个警告不代表程序有害，只是 macOS 的安全机制。程序代码完全开源，可以自行审查。

---

### 方式二：从源码编译

如果你想自己编译或进行二次开发：

#### 1. 安装 Rust

访问 https://rustup.rs/ 安装 Rust 工具链

#### 2. 克隆项目

```bash
git clone https://github.com/your_username/douyin_downloader.git
cd douyin_downloader
```

#### 3. 编译运行

```bash
# 开发模式运行
cargo run

# 或编译发布版本（更快）
cargo build --release
./target/release/douyin_downloader
```

#### macOS/Linux 用户额外步骤

下载后需要添加执行权限：

**macOS**:
```bash
chmod +x douyin_downloader-macos-x64
./douyin_downloader-macos-x64
```

**Linux**:
```bash
chmod +x douyin_downloader-linux-x64
./douyin_downloader-linux-x64
```

---

## 使用教程

### 1. 运行程序

**Windows**: 双击 `douyin_downloader-windows-x64.exe`  
**macOS/Linux**: 在终端中运行
```bash
./douyin_downloader-macos-x64  # macOS
./douyin_downloader-linux-x64  # Linux
```

### 2. 复制抖音分享内容

在抖音 App 中：
1. 找到想要下载的视频
2. 点击右下角「分享」按钮
3. 点击「复制链接」

你会得到类似这样的内容：
```
2.07 复制打开抖音，看看【xxx】我给大家跳个..  https://v.douyin.com/X21wMtxRtHP/ ipD:/ 04/27
```

### 3. 粘贴到程序中

```
=== 抖音视频下载工具 (支持多清晰度) ===

请粘贴抖音分享的内容 (输入 'q' 退出):
[粘贴刚才复制的内容]

✓ 提取到链接: https://v.douyin.com/X21wMtxRtHP/
正在解析视频地址...
  → 正在查找所有清晰度...
  ✓ 找到 4 个清晰度选项

可用的清晰度:
  1. 超清 1080P
  2. 高清 720P
  3. 标清 540P
  4. 流畅 360P

输入数字选择清晰度 (直接回车选择最高清晰度):
```

### 4. 选择清晰度

- **直接按回车**：自动选择最高清晰度（推荐）
- **输入数字 1-4**：选择指定清晰度

```
✓ 已选择: 超清 1080P

开始下载视频...
  下载进度: 100.0%
✓ 视频下载完成: video/douyin_1706342400.mp4
```

### 5. 继续下载或退出

下载完成后，可以继续粘贴新的分享链接，或输入 `q` 退出程序。

---

## 视频保存位置

保存到当前系统默认下载位置

---

## 高级功能

### 调试模式

如果遇到下载问题，可以启用调试模式：

**Windows (cmd)**:
```cmd
set DEBUG=1
douyin_downloader-windows-x64.exe
```

**macOS/Linux**:
```bash
DEBUG=1 ./douyin_downloader-macos-x64
```

这会生成 `debug_page.html` 文件，包含详细的调试信息。

### 批量下载

程序支持连续下载多个视频，只需一个接一个粘贴分享链接即可：

```
请粘贴抖音分享的内容:
[粘贴第一个视频]
✓ 视频下载完成: video/douyin_1.mp4

请粘贴抖音分享的内容:
[粘贴第二个视频]
✓ 视频下载完成: video/douyin_2.mp4

请粘贴抖音分享的内容:
q
再见!
```

---

## 支持的链接格式

✅ 标准短链接: `https://v.douyin.com/ABC123/`  
✅ 新格式短链接: `https://v.douyin.com/d-xxx-LQ/`  
✅ 完整视频链接: `https://www.douyin.com/video/数字ID`  
✅ 分享链接: `https://www.iesdouyin.com/share/video/数字ID`

详细说明请查看 [链接格式文档](LINK_FORMATS.md)

---

## 常见问题

### Q: 下载的视频有水印吗？

**A: 没有水印！** ✅

程序实现了多种无水印获取策略：
- 优先查找 `download_addr` 字段（通常无水印）
- 自动将 `/playwm/` 转换为 `/play/`
- 移除 URL 中的水印参数

所有清晰度的视频都是无水印版本。

详细说明请查看 [无水印下载文档](NO_WATERMARK.md)

---

### Q: 可以选择不同清晰度吗？

**A: 可以！** 🎬

程序会自动检测所有可用的清晰度（360P/540P/720P/1080P），你可以：
- 直接回车选择最高清晰度
- 输入数字选择指定清晰度

详细说明请查看 [多清晰度文档](MULTI_QUALITY.md)

---

### Q: 提示"无法从页面中提取视频地址"怎么办？

**A: 可能原因和解决方法**：

1. **抖音更新了页面结构**
   - 启用调试模式：`DEBUG=1` 运行程序
   - 检查项目是否有新版本更新

2. **网络问题**
   - 检查网络连接
   - 稍后重试

3. **视频特殊**
   - 视频可能被删除
   - 视频设置为私密
   - 视频需要登录观看

详细故障排除请查看 [故障排除文档](TROUBLESHOOTING.md)

---

### Q: macOS 提示"无法打开，因为无法验证开发者"？

**A: 这是正常的 macOS 安全提示**

请参考上面的 [macOS 用户注意事项](#macos-用户注意事项-) 部分，有三种解决方法。

---

### Q: Windows 提示"Windows 已保护你的电脑"？

**A: 点击"更多信息" → "仍要运行"**

这是 Windows SmartScreen 的安全提示，因为程序没有数字签名。

---

### Q: 可以批量下载吗？

**A: 可以！** ✅

程序支持循环输入，一个接一个粘贴分享内容即可。输入 `q` 退出程序。

---

### Q: 视频保存在哪里？

**A: 保存在 `video` 文件夹中**

- Windows: `douyin_downloader-windows-x64.exe` 所在目录下的 `video` 文件夹
- macOS/Linux: 程序所在目录下的 `video` 文件夹

文件名格式：`douyin_时间戳.mp4`

---

### Q: 支持哪些操作系统？

**A: 支持主流操作系统** 🖥️

- ✅ Windows 10/11 (64位)
- ✅ macOS 10.15+ (Intel 和 Apple Silicon)
- ✅ Linux (Ubuntu/Debian/Fedora 等)

---

## 技术实现

### 工作原理

1. **链接提取**: 使用正则表达式从分享文本中提取抖音链接
2. **地址解析**: 访问短链接，获取重定向后的长链接
3. **多清晰度检测**: 从页面的 `bit_rate` 数组中提取所有可用清晰度
4. **无水印处理**: 优先使用 `download_addr` 字段，自动转换 URL
5. **流式下载**: 使用异步流式下载，实时显示进度

### 技术栈

- **语言**: Rust
- **异步运行时**: tokio
- **HTTP 客户端**: reqwest
- **正则表达式**: regex
- **流处理**: futures-util

### 项目结构

```
douyin_downloader/
├── .github/
│   └── workflows/
│       └── build.yml        # GitHub Actions 自动构建配置
├── src/
│   ├── main.rs             # 主程序
│   └── debug_tool.rs       # 调试工具
├── tests/                   # 单元测试
├── Cargo.toml              # 项目配置
├── README.md               # 本文档
├── LINK_FORMATS.md         # 链接格式说明
├── MULTI_QUALITY.md        # 多清晰度说明
├── NO_WATERMARK.md         # 无水印说明
└── TROUBLESHOOTING.md      # 故障排除
```

---

## 开发者信息

### 参与贡献

欢迎提交 Issue 和 Pull Request！

1. Fork 本项目
2. 创建新分支: `git checkout -b feature/your-feature`
3. 提交更改: `git commit -m 'Add some feature'`
4. 推送分支: `git push origin feature/your-feature`
5. 提交 Pull Request

### 编译说明

```bash
# 开发构建
cargo build

# 发布构建（优化）
cargo build --release

# 运行测试
cargo test

# 运行调试工具
cargo run --bin debug_tool
```

### 发布新版本

项目使用 GitHub Actions 自动构建和发布：

```bash
# 创建新版本 tag
git tag v1.0.1 -m "Release v1.0.1"

# 推送 tag（自动触发构建）
git push origin v1.0.1
```

构建完成后会自动在 Releases 页面发布所有平台的可执行文件。

---

## 更新日志

### v1.0.0 (2025-01-28)

**新功能**:
- ✨ 支持多清晰度选择 (360P/540P/720P/1080P)
- ✨ 优先下载无水印版本
- ✨ 支持新的短链接格式（带连字符）
- ✨ 自动创建 `video` 文件夹保存视频
- ✨ 循环下载多个视频
- ✨ 跨平台支持（Windows/macOS/Linux）

**改进**:
- 🚀 使用 GitHub Actions 自动构建
- 🎨 优化命令行界面
- 📝 完善文档

---

## 注意事项

⚠️ **反爬虫机制**: 抖音有反爬虫机制，如果频繁请求可能被限制  
⚠️ **网络问题**: 如果下载失败，请检查网络连接或稍后重试  
⚠️ **合法使用**: 请遵守相关法律法规，仅用于个人学习和研究  
⚠️ **版权声明**: 下载的视频版权归原作者所有，请勿用于商业用途

---

## 许可证

MIT License

---

## 免责声明

本工具仅供学习交流使用，请勿用于商业用途。使用本工具下载的内容，版权归原作者所有。用户需自行承担使用本工具的所有风险和责任。

---

## 致谢

感谢所有为本项目做出贡献的开发者！

如果这个工具对你有帮助，欢迎给项目点个 ⭐ Star！

---

**最后更新**: 2025-01-28  
**版本**: v1.0.0