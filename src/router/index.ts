import { createRouter, createWebHashHistory, type RouteRecordRaw } from "vue-router";

const MODE_STORAGE_KEY = "rathole-launcher.mode";

function readSavedMode(): "server" | "client" {
  try {
    const saved = window.localStorage.getItem(MODE_STORAGE_KEY);
    if (saved === "server" || saved === "client") return saved;
  } catch {
    // localStorage may be unavailable
  }
  return "server";
}

const routes: RouteRecordRaw[] = [
  { path: "/", redirect: () => `/${readSavedMode()}` },
  {
    path: "/server",
    name: "server",
    component: () => import("@/views/WorkspaceView.vue"),
    props: { mode: "server" },
  },
  {
    path: "/server/:name",
    name: "server-detail",
    component: () => import("@/views/WorkspaceView.vue"),
    props: (route) => ({ mode: "server", name: route.params.name }),
  },
  {
    path: "/client",
    name: "client",
    component: () => import("@/views/WorkspaceView.vue"),
    props: { mode: "client" },
  },
  {
    path: "/client/:name",
    name: "client-detail",
    component: () => import("@/views/WorkspaceView.vue"),
    props: (route) => ({ mode: "client", name: route.params.name }),
  },
  {
    path: "/settings",
    name: "settings",
    component: () => import("@/views/SettingsView.vue"),
  },
];

export const router = createRouter({
  history: createWebHashHistory(),
  routes,
});

export function persistMode(mode: "server" | "client") {
  try {
    window.localStorage.setItem(MODE_STORAGE_KEY, mode);
  } catch {
    // ignore
  }
}
