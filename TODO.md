# middleTool 开发计划

## 当前状态
- ✅ 基础主题系统已实现（DbxPalette）
- ✅ 所有组件已集成主题系统
- ✅ 语句边界检测功能
- ✅ 改进Ctrl+Enter执行逻辑
- ✅ 可调整面板大小组件
- ✅ 数据表格改进（动态列宽、斑马条纹）
- ✅ 修复编辑器快捷键（Ctrl+Z/X/C/V）
- ✅ 添加等宽字体支持
- 🔄 进行中：UI重构与功能增强

## 第一阶段：主题系统基础 ✅
- [x] 创建主题模块 (`src/theme/`)
- [x] 定义DbxPalette颜色结构体
- [x] 提供可复用的样式函数
- [x] 更新所有视图和小组件使用主题

## 第二阶段：核心功能改进 ✅
- [x] 语句边界检测 (`src/editor/statement.rs`)
- [x] 改进Ctrl+Enter执行逻辑
- [x] 可调整面板大小 (`src/widgets/splitter.rs`)
- [x] 数据表格改进（动态列宽、斑马条纹）
- [x] 修复编辑器快捷键（Ctrl+Z/X/C/V）
- [x] 添加等宽字体支持

## 第三阶段：UI增强
- [ ] 标签页式查询结果
- [ ] 改进侧边栏树形结构
- [ ] 加载指示器
- [ ] 对话框覆盖层系统

## 第四阶段：SQL自动补全系统
- [ ] 添加依赖：sqlparser-rs, fuzzy-matcher
- [ ] 元数据缓存 (`src/autocomplete/metadata.rs`)
- [ ] 光标上下文检测 (`src/autocomplete/context.rs`)
- [ ] 候选生成 (`src/autocomplete/candidates.rs`)
- [ ] 补全引擎 (`src/autocomplete/engine.rs`)
- [ ] 补全弹窗 (`src/widgets/completion_popup.rs`)

## 技术决策
1. **使用sqlparser-rs替代ANTLR4**：纯Rust实现，无需外部运行时
2. **统一深色主题**：DbxPalette提供一致的颜色方案
3. **生命周期管理**：正确处理iced组件的生命周期

## 下一步行动
1. 实现标签页式查询结果
2. 改进侧边栏树形结构
3. 添加加载指示器
4. 实现对话框覆盖层系统

## 文件结构
```
src/
├── theme/
│   ├── mod.rs
│   ├── palette.rs      # DbxPalette颜色定义
│   ├── styles.rs       # 可复用样式函数
│   └── typography.rs   # 字体常量
├── editor/
│   ├── mod.rs
│   └── statement.rs    # 语句边界检测
├── autocomplete/
│   ├── mod.rs
│   ├── engine.rs       # 补全引擎
│   ├── context.rs      # 上下文检测
│   ├── candidates.rs   # 候选生成
│   └── metadata.rs     # 元数据缓存
└── widgets/
    ├── completion_popup.rs  # 补全弹窗
    ├── splitter.rs          # 可调整面板
    ├── loading.rs           # 加载指示器
    └── data_table.rs        # 动态列宽、斑马条纹
```
