import { type Ref, ref, onMounted, onBeforeUnmount } from "vue";

export function useReveal(threshold = 0.1): {
  elRef: Ref<HTMLElement | null>;
  isRevealed: Ref<boolean>;
} {
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
      { threshold },
    );
    observer.observe(elRef.value);
  });

  onBeforeUnmount(() => observer?.disconnect());

  return { elRef, isRevealed };
}
