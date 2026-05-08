import { createRouter, createWebHashHistory, type RouteRecordRaw } from "vue-router";

const routes: RouteRecordRaw[] = [
  { path: "/", redirect: "/server" },
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
