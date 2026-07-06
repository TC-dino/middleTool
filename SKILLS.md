# MiMo Code Skills 清单

> 自动生成于 2026-07-06

## 创意与设计类

| Skill | 描述 |
|-------|------|
| **brainstorming** | 创意工作前必须使用。探索用户意图、需求和设计，再进行实现。适用于创建功能、构建组件、添加功能或修改行为。 |
| **frontend-design** | 为新 UI 或重塑现有 UI 提供独特的视觉设计指导。帮助确定美学方向、排版和做出不模板化的选择。 |

## 编程语言专项类

| Skill | 描述 |
|-------|------|
| **rust-best-practices** | 基于 Apollo GraphQL 最佳实践手册的 Rust 编写指南。适用于：编写新代码、审查/重构代码、借用 vs 克隆决策、Result 错误处理、性能优化、测试/文档编写。 |
| **rust-async-patterns** | 掌握 Rust 异步编程，包括 Tokio、async traits、错误处理和并发模式。适用于构建异步应用、实现并发系统或调试异步代码。 |
| **rust-testing** | Rust 测试模式，包括单元测试、集成测试、异步测试、基于属性的测试、mock 和覆盖率。遵循 TDD 方法论。 |
| **m06-error-handling** | 错误处理专项。触发词：Result, Option, Error, ?, unwrap, expect, panic, anyhow, thiserror 等。何时用 panic vs Result，自定义错误，错误传播。 |

## 工作流与方法论类

| Skill | 描述 |
|-------|------|
| **test-driven-development** | 实现任何功能或 bugfix 前使用，先写测试再写实现。 |
| **writing-plans** | 当你有规格说明或多步任务需求时，在动手写代码前使用。 |
| **systematic-debugging** | 遇到任何 bug、测试失败或意外行为时使用，在提出修复方案前先系统排查。 |
| **requesting-code-review** | 完成任务、实现主要功能或合并前使用，验证工作符合要求。 |
| **using-superpowers** | 任何对话开始时使用。建立如何查找和使用 skills 的规则。 |

## 通信与效率类

| Skill | 描述 |
|-------|------|
| **caveman** | 超压缩通信模式。通过洞穴人式说话方式节省约 75% token，同时保持完整技术准确性。支持 lite、full（默认）、ultra 三级强度。 |

## 工具与扩展类

| Skill | 描述 |
|-------|------|
| **find-skills** | 帮助用户发现和安装 agent skills。当用户问"如何做 X"、"找一个 X 的 skill"时使用。 |
| **self-extend** | 自我进化接口。创建新工具避免重复模式、添加钩子改进自身行为、构建 skills 积累领域知识，或覆盖内置工具以适应项目需求。 |

## 语音合成类

| Skill | 描述 |
|-------|------|
| **mimo-v2-5-tts** | MiMo V2.5 TTS 语音合成。使用小米 MiMo V2.5 TTS 系列模型生成语音。支持预置音色、音色设计、音色克隆三种模式，支持自然语言控制、导演模式、语气/情绪/方言风格标签，预置音色支持唱歌。 |

---

**共计 15 个 Skills**

### 调用方式

- 输入 `/<skill-name>` 即可调用对应 skill（如 `/brainstorming`）
- 直接说出 skill 名称也会自动激活（如 "caveman mode"）
- 部分 skill 会在特定场景自动触发（如 `using-superpowers` 在对话开始时）

### 来源路径

| 来源 | 路径 |
|------|------|
| 用户自定义 | `~/.claude/skills/`、`~/.agents/skills/` |
| 内置 builtin | `~/.local/share/mimocode/builtin_skills/` |
