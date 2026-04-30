<script lang="ts" setup>
import { ref, onMounted, onUnmounted, watch, nextTick } from "vue";

export interface AnimatedListItem {
  id: string;
  label: string;
  subLabel?: string;
}

const props = withDefaults(
  defineProps<{
    items?: AnimatedListItem[];
    showGradients?: boolean;
    enableArrowNavigation?: boolean;
    initialSelectedIndex?: number;
    selectedIds?: Set<string>;
  }>(),
  {
    items: () => [],
    showGradients: true,
    enableArrowNavigation: true,
    initialSelectedIndex: -1,
    selectedIds: () => new Set(),
  },
);

const emit = defineEmits<{
  itemSelected: [item: AnimatedListItem, index: number];
}>();

const listRef = ref<HTMLElement | null>(null);
const selectedIndex = ref(props.initialSelectedIndex);
const keyboardNav = ref(false);
const topGradientOpacity = ref(0);
const bottomGradientOpacity = ref(1);
const itemsInView = ref<boolean[]>([]);

const getItemInView = (index: number) => itemsInView.value[index] ?? false;

const handleScroll = (e: Event) => {
  const el = e.target as HTMLElement;
  const { scrollTop, scrollHeight, clientHeight } = el;
  topGradientOpacity.value = Math.min(scrollTop / 50, 1);
  const bottomDist = scrollHeight - (scrollTop + clientHeight);
  bottomGradientOpacity.value =
    scrollHeight <= clientHeight ? 0 : Math.min(bottomDist / 50, 1);
  updateItemsInView();
};

const updateItemsInView = () => {
  if (!listRef.value) return;
  const container = listRef.value;
  const cr = container.getBoundingClientRect();
  itemsInView.value = props.items.map((_, index) => {
    const item = container.querySelector(`[data-index="${index}"]`) as HTMLElement | null;
    if (!item) return false;
    const ir = item.getBoundingClientRect();
    const top = ir.top - cr.top;
    const bottom = top + ir.height;
    return top < cr.height && bottom > 0;
  });
};

const selectAndEmit = (index: number) => {
  selectedIndex.value = index;
  emit("itemSelected", props.items[index], index);
};

const handleKeyDown = (e: KeyboardEvent) => {
  if (!props.enableArrowNavigation) return;
  if (e.key === "ArrowDown" || (e.key === "Tab" && !e.shiftKey)) {
    e.preventDefault();
    keyboardNav.value = true;
    selectedIndex.value = Math.min(selectedIndex.value + 1, props.items.length - 1);
  } else if (e.key === "ArrowUp" || (e.key === "Tab" && e.shiftKey)) {
    e.preventDefault();
    keyboardNav.value = true;
    selectedIndex.value = Math.max(selectedIndex.value - 1, 0);
  } else if (e.key === "Enter" && selectedIndex.value >= 0) {
    e.preventDefault();
    selectAndEmit(selectedIndex.value);
  }
};

watch([selectedIndex, keyboardNav], () => {
  if (!keyboardNav.value || selectedIndex.value < 0 || !listRef.value) return;
  const container = listRef.value;
  const item = container.querySelector(`[data-index="${selectedIndex.value}"]`) as HTMLElement | null;
  if (!item) return;
  const extra = 50;
  const { scrollTop, clientHeight } = container;
  const top = item.offsetTop;
  const bottom = top + item.offsetHeight;
  if (top < scrollTop + extra) {
    container.scrollTo({ top: top - extra, behavior: "smooth" });
  } else if (bottom > scrollTop + clientHeight - extra) {
    container.scrollTo({ top: bottom - clientHeight + extra, behavior: "smooth" });
  }
  keyboardNav.value = false;
});

watch(
  () => props.items.length,
  () => {
    nextTick(() => {
      updateItemsInView();
    });
  },
);

onMounted(() => {
  if (props.enableArrowNavigation) window.addEventListener("keydown", handleKeyDown);
  requestAnimationFrame(() => {
    updateItemsInView();
  });
});

onUnmounted(() => {
  if (props.enableArrowNavigation) window.removeEventListener("keydown", handleKeyDown);
});
</script>

<template>
  <div class="animated-list-wrapper">
    <div
      ref="listRef"
      class="animated-list-scroll"
      @scroll="handleScroll"
    >
      <div
        v-for="(item, index) in items"
        :key="item.id"
        :data-index="index"
        :class="[
          'animated-list-item',
          {
            'in-view': getItemInView(index),
            selected: selectedIndex === index,
            checked: selectedIds.has(item.id),
          },
        ]"
        @mouseenter="selectedIndex = index"
        @click="selectAndEmit(index)"
      >
        <div class="animated-list-item-inner">
          <div class="animated-list-item-check">
            <svg
              v-if="selectedIds.has(item.id)"
              class="check-icon" width="18" height="18" viewBox="0 0 18 18" fill="none"
            >
              <rect x="0.5" y="0.5" width="17" height="17" rx="5" fill="currentColor" opacity="0.15"/>
              <rect x="0.5" y="0.5" width="17" height="17" rx="5" stroke="currentColor" stroke-width="1.5"/>
              <path d="M5 9.5l2.5 2.5L13 6" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
            <svg
              v-else
              class="check-icon" width="18" height="18" viewBox="0 0 18 18" fill="none"
            >
              <rect x="0.5" y="0.5" width="17" height="17" rx="5" stroke="currentColor" stroke-width="1.2" opacity="0.2"/>
            </svg>
          </div>
          <div class="animated-list-item-text">
            <span class="animated-list-item-label">{{ item.label }}</span>
            <span v-if="item.subLabel" class="animated-list-item-sublabel">{{ item.subLabel }}</span>
          </div>
        </div>
      </div>
    </div>
    <div
      v-if="showGradients"
      class="animated-list-gradient top"
      :style="{ opacity: topGradientOpacity }"
    />
    <div
      v-if="showGradients"
      class="animated-list-gradient bottom"
      :style="{ opacity: bottomGradientOpacity }"
    />
  </div>
</template>

<style scoped>
.animated-list-wrapper {
  position: relative;
  width: 100%;
}

.animated-list-scroll {
  max-height: 400px;
  overflow-y: auto;
  padding: 4px 0;
  scrollbar-width: thin;
  scrollbar-color: var(--border) transparent;
}
.animated-list-scroll::-webkit-scrollbar { width: 8px; }
.animated-list-scroll::-webkit-scrollbar-track { background: transparent; }
.animated-list-scroll::-webkit-scrollbar-thumb {
  background: var(--border-strong);
  border-radius: 4px;
}

.animated-list-item {
  margin-bottom: 6px;
  cursor: pointer;
  opacity: 0.5;
  transform: scale(0.7);
  transition:
    opacity 0.25s cubic-bezier(0.16, 1, 0.3, 1),
    transform 0.25s cubic-bezier(0.16, 1, 0.3, 1),
    background 0.15s ease;
  border-radius: 10px;
}
.animated-list-item.in-view {
  opacity: 1;
  transform: scale(1);
}
.animated-list-item:last-child {
  margin-bottom: 0;
}

.animated-list-item-inner {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 12px 14px;
  background: var(--surface);
  border: 1px solid var(--border);
  border-radius: 10px;
  transition:
    background 0.15s ease,
    border-color 0.15s ease;
  position: relative;
  overflow: hidden;
}
.animated-list-item.checked .animated-list-item-inner {
  background: rgba(var(--accent-cool-rgb), 0.06);
  border-color: rgba(var(--accent-cool-rgb), 0.35);
}
.animated-list-item.checked .animated-list-item-inner::before {
  content: '';
  position: absolute;
  left: 0;
  top: 0;
  bottom: 0;
  width: 3px;
  background: var(--accent-cool);
  border-radius: 0 2px 2px 0;
}
.animated-list-item:hover .animated-list-item-inner {
  background: var(--surface-hover);
  border-color: var(--border-strong);
}
.animated-list-item.checked:hover .animated-list-item-inner {
  background: rgba(var(--accent-cool-rgb), 0.09);
  border-color: rgba(var(--accent-cool-rgb), 0.45);
}
.animated-list-item.selected .animated-list-item-inner {
  background: var(--accent-cool-soft);
  border-color: rgba(var(--accent-cool-rgb), 0.3);
}

.animated-list-item-check {
  flex-shrink: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  width: 20px;
  height: 20px;
}
.animated-list-item.checked .check-icon {
  color: var(--accent-cool);
}
.check-icon {
  color: var(--text-4);
  display: block;
}

.animated-list-item-text {
  display: flex;
  flex-direction: column;
  gap: 2px;
  min-width: 0;
}

.animated-list-item-label {
  font-size: 0.875rem;
  font-weight: 600;
  color: var(--text);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.animated-list-item-sublabel {
  font-size: 0.75rem;
  color: var(--text-3);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.animated-list-gradient {
  position: absolute;
  left: 0;
  right: 0;
  height: 40px;
  pointer-events: none;
  z-index: 2;
  transition: opacity 0.2s ease;
}
.animated-list-gradient.top {
  top: 0;
  background: linear-gradient(to bottom, var(--surface), transparent);
}
.animated-list-gradient.bottom {
  bottom: 0;
  background: linear-gradient(to top, var(--surface), transparent);
}
</style>
