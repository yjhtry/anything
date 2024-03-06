<script setup lang="ts">
const route = useRoute()
const router = useRouter()
const navList = ref([
  { name: 'package', path: '/package' },
  { name: 'oss', path: '/oss' },
])

function openLink(path: string) {
  router.push(path)
}
</script>

<template>
  <nav
    text-xl class="nav flex border-b-1 border-b-[#e2e8f0] border-b-solid bg-white px-6 py-3 dark:border-b-[#27272a] dark:bg-[rgb(24,_24,_27)]"
  >
    <div class="mr-auto flex items-center gap-3 font-700" text="4">
      <Avatar image="/logo.png" />
      Anything
    </div>
    <div class="flex items-center">
      <ul text-sm font-600 class="flex items-center gap-5">
        <li
          v-for="nav in navList"
          :key="nav.name"
          class="inline-block cursor-pointer hover:text-sky-400"
          :class="{ 'text-sky-400': route.path === nav.path }"
          @click="openLink(nav.path)"
        >
          {{ nav.name }}
        </li>
      </ul>
      <div
        icon-btn
        class="ml-8 flex items-center gap-3 border-l border-l-slate-200 border-l-solid pl-6"
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
