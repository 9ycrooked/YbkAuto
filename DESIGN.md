# DESIGN.md

> 一个安静、克制的桌面工具。像 macOS 设置面板一样，不喧宾夺主，让课程数据自己说话。

## 1. Visual Theme & Atmosphere

**Style**: Apple HIG Dashboard
**Keywords**: 克制、半透明、微妙层次、系统原生感、冷灰底、高密度信息
**Tone**: 专业工具 — NOT 赛博朋克、NOT 毛玻璃特效堆叠、NOT 暗黑霓虹
**Feel**: 像打开 macOS 的系统设置或 Xcode Organizer——你几乎注意不到设计的存在，但每个像素都在正确的位置。

**Interaction Tier**: L2 流畅交互
**Dependencies**: CSS only（IntersectionObserver + CSS transitions，不引入 GSAP/ScrollTrigger）

## 2. Color Palette & Roles

系统跟随 Light / Dark 模式。所有颜色通过 CSS 变量引用，零硬编码 hex。

```css
/* ── Light Mode ── */
:root {
  --bg: #f5f5f7;                       /* 主背景（Apple 经典暖灰白） */
  --bg-rgb: 245, 245, 247;
  --surface: #ffffff;                  /* 卡片 / 容器 */
  --surface-rgb: 255, 255, 255;
  --surface-hover: #fafafa;            /* 悬停表面 */
  --border: #e5e5ea;                   /* 默认边框 */
  --border-rgb: 229, 229, 234;
  --border-hover: #d1d1d6;             /* 悬停边框 */

  --text: #1d1d1f;                     /* 标题 / 重要文字 */
  --text-rgb: 29, 29, 31;
  --text-secondary: #6e6e73;           /* 正文 / 描述 */
  --text-tertiary: #aeaeb2;            /* 标签 / 辅助 */

  --accent: #007aff;                   /* 主强调色（Apple Blue） */
  --accent-rgb: 0, 122, 255;
  --accent-hover: #0066d6;

  --success: #34c759;                  /* 已完成 / 开放 */
  --success-rgb: 52, 199, 89;
  --warning: #ff9f0a;                  /* 进行中 / 未开始 */
  --warning-rgb: 255, 159, 10;
  --error: #ff3b30;                    /* 异常 / 错误 */
  --error-rgb: 255, 59, 48;

  --shadow-color: 0, 0, 0;
  --shadow-strength: 4%;
}

/* ── Dark Mode ── */
@media (prefers-color-scheme: dark) {
  :root {
    --bg: #1c1c1e;                     /* Apple 暗色系统背景 */
    --bg-rgb: 28, 28, 30;
    --surface: #2c2c2e;                /* 卡片 */
    --surface-rgb: 44, 44, 46;
    --surface-hover: #3a3a3c;          /* 悬停 */
    --border: #38383a;                 /* 边框 */
    --border-rgb: 56, 56, 58;
    --border-hover: #48484a;

    --text: #f5f5f7;                   /* 标题 */
    --text-rgb: 245, 245, 247;
    --text-secondary: #a1a1a6;         /* 正文 */
    --text-tertiary: #636366;          /* 辅助 */

    --accent: #0a84ff;                 /* 暗色模式 Blue */
    --accent-hover: #409cff;

    --success: #30d158;
    --warning: #ffd60a;
    --warning-rgb: 255, 214, 10;
    --error: #ff453a;

    --shadow-strength: 20%;
  }
}
```

**Color Rules:**
- 所有颜色通过 CSS 变量引用，禁止硬编码 hex/rgb
- 同一个 section 内只用 accent + success/warning/error 中的必要色，不混搭装饰色
- 不添加额外"装饰性强调色"——accent 就是唯一的品牌色
- 渐变仅用于暗色模式的 background 氛围层，不用于组件边框或文字

## 3. Typography Rules

**Font Stack**（系统原生优先，中文 fallback）：

```css
@import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Noto+Sans+SC:wght@400;500;600;700&display=swap');

body {
  font-family:
    'Inter', 'Noto Sans SC',
    -apple-system, BlinkMacSystemFont,
    'SF Pro Text', 'SF Pro Display',
    'PingFang SC', 'Microsoft YaHei',
    sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  letter-spacing: -0.01em;
}
```

| Role | Font | Size | Weight | Line Height | Letter Spacing |
|------|------|------|--------|-------------|----------------|
| Hero / Page H1 | Inter | 1.75rem (28px) | 700 | 1.2 | -0.02em |
| Section H2 / Card title | Inter | 1.125rem (18px) | 600 | 1.3 | -0.01em |
| KPI Number | Inter | 1.5rem (24px) | 700 | 1.2 | -0.02em |
| Body | Inter / Noto Sans SC | 0.875rem (14px) | 400 | 1.5 | — |
| Label / Caption | Inter / Noto Sans SC | 0.75rem (12px) | 500 | 1.4 | 0.02em |
| Mono / Code | SF Mono / Menlo | 0.8125rem (13px) | 400 | 1.5 | — |

**Typography Rules:**
- 中文行高 ≥ 1.5，英文 ≥ 1.3
- Heading weight ≥ 600
- 不混用衬线字体——全部 sans
- **NEVER use**: 装饰性 script 字体、超过 2 个字重、`text-transform: uppercase` 用于中文

**Text Decoration:**
- Hero h1: 无渐变、无投影（克制风格，禁止装饰）
- Section h2: 无投影
- 正文 p: 禁止任何装饰

## 4. Component Stylings

### Buttons

```css
.btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
  min-height: 36px;
  padding: 0 16px;
  border: none;
  border-radius: 10px;
  font-family: inherit;
  font-size: 0.875rem;
  font-weight: 500;
  line-height: 1;
  cursor: pointer;
  user-select: none;
  transition: all 0.18s ease;
}
.btn:focus-visible {
  outline: 2px solid var(--accent);
  outline-offset: 2px;
}
.btn:disabled {
  opacity: 0.4;
  cursor: not-allowed;
}

/* Primary */
.btn-primary {
  background: var(--accent);
  color: #ffffff;
}
.btn-primary:hover:not(:disabled) {
  background: var(--accent-hover);
  transform: translateY(-1px);
  box-shadow: 0 2px 8px rgba(var(--accent-rgb), 0.3);
}
.btn-primary:active:not(:disabled) {
  transform: translateY(0) scale(0.97);
  box-shadow: none;
}

/* Secondary / Ghost */
.btn-ghost {
  background: transparent;
  color: var(--text);
  border: 1px solid var(--border);
}
.btn-ghost:hover:not(:disabled) {
  background: var(--surface-hover);
  border-color: var(--border-hover);
}

/* Destructive (logout, etc.) */
.btn-ghost-danger {
  background: transparent;
  color: var(--error);
  border: 1px solid var(--border);
}
.btn-ghost-danger:hover:not(:disabled) {
  background: rgba(var(--error-rgb), 0.08);
  border-color: var(--error);
}
```

### Cards (Course Card)

```css
.course-card {
  background: var(--surface);
  border: 1px solid var(--border);
  border-radius: 16px;
  padding: 20px;
  box-shadow: 0 1px 3px rgba(var(--shadow-color), calc(var(--shadow-strength) * 1));
  transition:
    transform 0.25s cubic-bezier(0.16, 1, 0.3, 1),
    box-shadow 0.25s cubic-bezier(0.16, 1, 0.3, 1),
    border-color 0.2s ease;
}
.course-card:hover {
  transform: translateY(-2px);
  border-color: var(--border-hover);
  box-shadow:
    0 4px 12px rgba(var(--shadow-color), calc(var(--shadow-strength) * 2)),
    0 1px 3px rgba(var(--shadow-color), calc(var(--shadow-strength) * 1));
}
.course-card:focus-visible {
  outline: 2px solid var(--accent);
  outline-offset: 2px;
}
```

### Summary Strip

```css
.summary-strip {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 2px;
  background: var(--border);
  border-radius: 16px;
  overflow: hidden;
  border: 1px solid var(--border);
}
.summary-item {
  background: var(--surface);
  padding: 18px 22px;
  display: flex;
  flex-direction: column;
  gap: 4px;
}
.summary-label {
  font-size: 0.75rem;
  font-weight: 500;
  color: var(--text-tertiary);
  text-transform: uppercase;
  letter-spacing: 0.04em;
}
.summary-value {
  font-size: 1.5rem;
  font-weight: 700;
  color: var(--text);
  letter-spacing: -0.02em;
}
.summary-caption {
  font-size: 0.75rem;
  color: var(--text-tertiary);
}
```

### Status Chips

```css
.status-chip {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  min-height: 24px;
  padding: 0 10px;
  border-radius: 999px;
  font-size: 0.75rem;
  font-weight: 600;
  line-height: 1;
  letter-spacing: 0.01em;
}
/* Completed / Open */
.status-chip--success {
  background: rgba(var(--success-rgb), 0.12);
  color: var(--success);
}
/* In Progress / Pending */
.status-chip--warning {
  background: rgba(var(--warning-rgb), 0.12);
  color: var(--warning);
}
/* Error / Closed */
.status-chip--muted {
  background: rgba(var(--text-rgb), 0.07);
  color: var(--text-secondary);
}
```

### Toast Message

```css
.toast {
  position: fixed;
  top: 20px;
  left: 50%;
  transform: translateX(-50%);
  z-index: 9999;
  display: inline-flex;
  align-items: center;
  gap: 10px;
  max-width: min(380px, calc(100vw - 32px));
  min-height: 44px;
  padding: 0 12px;
  border-radius: 12px;
  background: var(--surface);
  border: 1px solid var(--border);
  box-shadow: 0 2px 12px rgba(var(--shadow-color), calc(var(--shadow-strength) * 2));
}
.toast__icon {
  flex-shrink: 0;
  display: flex;
  align-items: center;
  color: var(--error); /* or success/info */
}
.toast__message {
  flex: 1;
  font-size: 0.875rem;
  font-weight: 500;
  line-height: 1.4;
  color: var(--text);
}
.toast__close {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  border: none;
  border-radius: 8px;
  background: transparent;
  color: var(--text-4);
  cursor: pointer;
  transition: background 0.15s ease, color 0.15s ease;
}
.toast__close:hover {
  background: var(--surface-hover);
  color: var(--text-2);
}

.toast-enter-active {
  animation: toastIn 0.22s var(--ease-cinema) forwards;
}
.toast-leave-active {
  animation: toastOut 0.18s var(--ease) forwards;
}
@keyframes toastIn {
  from { opacity: 0; transform: translateX(-50%) translateY(-8px); }
  to { opacity: 1; transform: translateX(-50%) translateY(0); }
}
@keyframes toastOut {
  from { opacity: 1; transform: translateX(-50%) translateY(0); }
  to { opacity: 0; transform: translateX(-50%) translateY(-6px); }
}
```

**Rules:**
- 仅使用 `opacity` + `transform` 实现动画（无 scale）
- 入场 220ms / 退场 180ms（exit faster than enter）
- 自动消失时间 4s
- 无 `backdrop-filter`——纯 surface + 边框
- `z-index: 9999`，固定在最顶层

### Info Row (key-value inside cards)

```css
.info-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
}
.info-label {
  font-size: 0.8125rem;
  font-weight: 500;
  color: var(--text-secondary);
}
.resource-pill {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  min-height: 28px;
  padding: 0 12px;
  border-radius: 999px;
  font-size: 0.8125rem;
  font-weight: 500;
  background: rgba(var(--text-rgb), 0.05);
  color: var(--text-secondary);
}
```

### Dashboard Header

```css
.dashboard-header {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 16px;
  padding: 24px 0;
}
.header-user-info h1 {
  font-size: 1.75rem;
  font-weight: 700;
  color: var(--text);
  margin: 0 0 4px;
  letter-spacing: -0.02em;
}
.header-meta {
  font-size: 0.8125rem;
  color: var(--text-tertiary);
}
```

## 5. Layout Principles

**Container:**
- Max width: 1100px（与当前保持一致）
- Padding: 页面边缘 32px（移动端 18px）
- 内容区无额外 padding（由组件自身 padding 控制）

**Spacing Scale:**
- Section gap（header ↔ summary ↔ grid）: 24px
- Card grid gap: 16px
- Card internal padding: 20px
- Summary strip internal padding: 18px 22px

**Grid:**
```css
.course-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
  gap: 16px;
}
```

## 6. Depth & Elevation

禁止大面积 `backdrop-filter: blur()` ——这是当前设计最大的问题（模糊覆盖大面积导致性能差且视觉混乱）。

| Level | Treatment | Use |
|-------|-----------|-----|
| Flat | 无阴影，纯背景 + 边框 | 页面背景、section 内分隔 |
| Subtle | `box-shadow: 0 1px 3px rgba(0,0,0, 0.04)` | 默认卡片 |
| Elevated | `box-shadow: 0 4px 12px rgba(0,0,0, 0.08)` + `translateY(-2px)` | 卡片 hover |
| Modal | `box-shadow: 0 20px 60px rgba(0,0,0, 0.15)` + `backdrop-filter: blur(8px)` | 登录卡片（仅此一处） |

**Anti-pattern**: 不在大面积容器上使用 backdrop-filter blur。登录表单的模糊仅作用于卡片自身背景（面积为 320×300px 级别），且值 ≤ 8px。

## 7. Animation & Interaction

**Motion Philosophy**: 含蓄、精准。动画用于帮助理解信息层级而非炫技。只用 `opacity` 和 `transform`，零 `filter: blur()` 动画。

**Tier**: L2 流畅交互

### Dependencies
无外部依赖。全部通过 CSS transitions + IntersectionObserver 实现。

### Base Setup (Vue 3 composable)

```typescript
// src/composables/useReveal.ts
import { type Ref, ref, onMounted, onBeforeUnmount } from 'vue';

export function useReveal(threshold = 0.1): { elRef: Ref<HTMLElement | null>; isRevealed: Ref<boolean> } {
  const elRef = ref<HTMLElement | null>(null);
  const isRevealed = ref(false);
  let observer: IntersectionObserver | null = null;

  onMounted(() => {
    if (!elRef.value) return;
    observer = new IntersectionObserver(
      ([entry]) => {
        if (entry.isIntersecting) {
          isRevealed.value = true;
          observer?.unobserve(entry.target);
        }
      },
      { threshold }
    );
    observer.observe(elRef.value);
  });

  onBeforeUnmount(() => observer?.disconnect());

  return { elRef, isRevealed };
}
```

### Entrance — Course Card Stagger

卡片入场使用 CSS transition + staggered delay：

```css
.course-card {
  opacity: 0;
  transform: translateY(16px);
  transition:
    opacity 0.5s cubic-bezier(0.16, 1, 0.3, 1),
    transform 0.5s cubic-bezier(0.16, 1, 0.3, 1);
}
.course-card.in-view {
  opacity: 1;
  transform: translateY(0);
}
```

```css
/* Stagger via data-index (set by Vue v-for) */
.course-card[data-index="0"] { transition-delay: 0.0s; }
.course-card[data-index="1"] { transition-delay: 0.06s; }
.course-card[data-index="2"] { transition-delay: 0.12s; }
.course-card[data-index="3"] { transition-delay: 0.18s; }
.course-card[data-index="4"] { transition-delay: 0.24s; }
.course-card[data-index="5"] { transition-delay: 0.30s; }
.course-card[data-index="6"] { transition-delay: 0.36s; }
.course-card[data-index="7"] { transition-delay: 0.42s; }
/* cap at ~0.42s for larger lists */
```

### Summary Strip Entrance

```css
.summary-strip {
  opacity: 0;
  transform: translateY(-8px);
  animation: summaryIn 0.5s 0.15s cubic-bezier(0.16, 1, 0.3, 1) forwards;
}
@keyframes summaryIn {
  to {
    opacity: 1;
    transform: translateY(0);
  }
}
```

### Hover & Focus States（所有可交互元素）

| Element | Default | Hover | Focus |
|---------|---------|-------|-------|
| Course Card | `border: var(--border)` | `translateY(-2px)` + border 加深 + 阴影增强 | `outline: 2px var(--accent)` |
| Button Primary | `bg: var(--accent)` | `bg: var(--accent-hover)` + `translateY(-1px)` | `outline: 2px var(--accent)` |
| Button Ghost | `bg: transparent` + border | `bg: var(--surface-hover)` | `outline: 2px var(--accent)` |
| Status Chip | 静态 | 无 hover（非交互元素） | — |
| Refresh Button | 静态 | 旋转图标（通过 `isRefreshing` 状态） | `outline: 2px var(--accent)` |

### Loading State

```css
@keyframes shimmer {
  0% { background-position: -200% 0; }
  100% { background-position: 200% 0; }
}
.skeleton {
  background: linear-gradient(
    90deg,
    rgba(var(--text-rgb), 0.05) 25%,
    rgba(var(--text-rgb), 0.08) 50%,
    rgba(var(--text-rgb), 0.05) 75%
  );
  background-size: 200% 100%;
  animation: shimmer 1.5s ease-in-out infinite;
  border-radius: 8px;
}
```

| Animation | Duration | Easing / Notes |
|-----------|----------|----------------|
| Loading State | shimmer skeleton | `@keyframes shimmer` 1.5s ease-in-out infinite |
| Toast | 入场 220ms / 退场 180ms | `toastIn` / `toastOut`，exit faster than enter |

### Reduced Motion

```css
@media (prefers-reduced-motion: reduce) {
  *, *::before, *::after {
    animation-duration: 0.01ms !important;
    animation-iteration-count: 1 !important;
    transition-duration: 0.01ms !important;
  }
  .course-card {
    opacity: 1;
    transform: none;
    transition-delay: 0s !important;
  }
  .summary-strip {
    opacity: 1;
    transform: none;
    animation: none !important;
  }
  .toast-enter-active,
  .toast-leave-active {
    animation-duration: 0.01ms;
  }
}
```

## 8. Do's and Don'ts

### Do
- 用 `var(--text)` / `var(--surface)` 等变量引用所有颜色
- 用 `cubic-bezier(0.16, 1, 0.3, 1)` 作为默认缓动——这是 Apple 的"弹性但不跳"曲线
- 卡片 hover 用 `translateY(-1px/-2px)` + 阴影增强，不用 scale
- 状态徽标用纯色背景 + 文字，不用毛玻璃
- 信息密度保持在"看一眼就知道状态"——summary strip 4 列，卡片最多 5-6 行信息
- 中文文字 letter-spacing 设 0.01em-0.02em（标签级用 0.04em）
- 所有数字使用 tabular-nums（等宽数字，KPI 数值对齐）
- Light 和 Dark 模式保持完全相同的布局和间距，只切换颜色

### Don't
- ❌ 大面积使用 `backdrop-filter: blur()`——仅登录卡片可用，值 ≤ 8px
- ❌ Toast 组件使用 `backdrop-filter`——纯 surface + 边框即可
- ❌ 在移动元素上使用 `filter: blur()` 做景深——用 opacity + scale 替代
- ❌ 硬编码任何 hex 颜色值到组件中——必须通过 CSS 变量
- ❌ 使用超过 2 个强调色——accent 只有蓝色一个
- ❌ 给不同课程卡片使用不同颜色——所有卡片视觉一致，靠数据区分
- ❌ 在卡片内容区添加第二个背景色——卡片只有一个 surface 色
- ❌ 使用 `text-shadow` 或 `gradient text`——装饰性文字效果破坏工具感
- ❌ 卡片 hover 使用 scale——translateY(-2px) 即可，scale 会导致内容模糊
- ❌ 在暗色模式下背景使用纯黑 `#000000`——Apple 暗色背景是 `#1c1c1e`
- ❌ 在 summary strip 中使用彩色背景——只允许白色/暗色 surface

## 9. Responsive Behavior

**Breakpoints:**
| Name | Width | Key Changes |
|------|-------|-------------|
| Desktop | > 900px | 4 列 summary，auto-fill 卡片 grid，header 横排 |
| Mobile | ≤ 900px | 1 列 summary，header 竖排，卡片全宽 |

**Touch Targets:** minimum 44×44px（按钮、链接）

**Collapsing Strategy:**
- Summary strip: 4 列 → 1 列（纵向堆叠）
- Dashboard header: 用户信息 + 操作按钮横排 → 竖排堆叠
- Course cards: `auto-fill` 自动调整列数
- 卡片内部 info-row: 横排 → 竖排（label 在上，value 在下）

```css
@media (max-width: 900px) {
  .app-shell {
    padding: 18px;
  }

  .dashboard-header {
    flex-direction: column;
    align-items: flex-start;
  }

  .header-actions {
    width: 100%;
    flex-wrap: wrap;
  }

  .summary-strip {
    grid-template-columns: 1fr;
  }

  .course-card__top,
  .info-row {
    flex-direction: column;
    align-items: flex-start;
  }
}
```
