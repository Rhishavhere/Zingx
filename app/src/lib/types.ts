export type AppMode = "shell" | "chats" | "utils" | "host";

export interface ChatThread {
  id: string;
  title: string;
  updatedAt: string;
}
