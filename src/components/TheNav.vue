<template>
  <nav
    text-xl class="nav flex border-b-1 border-b-[#e2e8f0] border-b-solid bg-white px-6 py-3 dark:border-b-[#27272a] dark:bg-[rgb(24,_24,_27)]"
  >
    <div mr-auto text="4" font-700>
      Anything
    </div>
    <div
      icon-btn
      class="flex items-center gap-3"
      @click="toggleDark()"
    >
      <div text="4" i-carbon-sun dark:i-carbon-moon />
      <a
        i-carbon-logo-github icon-btn
        rel="noreferrer"
        href="https://github.com/yjhtry/anything"
        target="_blank"
        title="GitHub"
      />
    </div>
  </nav>
</template>

<style scoped>
.nav {
  position: sticky;
  left: 0;
  top: 0;
  z-index: 1100;
  backdrop-filter: blur(8px);
  width: calc(100% - var(--scrollbar-width, 0px));
  transition:
    background-color 0.5s,
    border-color 0.5s;
}
</style>
