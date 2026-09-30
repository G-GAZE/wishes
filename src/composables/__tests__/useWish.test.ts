import { invoke } from "@tauri-apps/api/core";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { useWish } from "../useWish";


vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(),
}));

// 单测不处于组件 setup 上下文, mock 掉 i18n (仅测试文案拼装行为)
vi.mock("vue-i18n", () => ({
  useI18n: () => ({
    t: (key: string, params?: Record<string, unknown>) =>
      params ? `${key} ${Object.values(params).map(String).join(" ")}` : key,
  }),
}));

describe("useWish", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("should handle successful single wish", async () => {
    const mockResponse = { id: 1, content: 'Test-Card', tags: [], event_tags: [] };
    (invoke as any).mockResolvedValue(mockResponse);

    const { doWish, currentWish, loading, history } = useWish();
    const promise = doWish(1);

    expect(loading.value).toBe(true);

    await promise;

    expect(loading.value).toBe(false);
    expect(currentWish.value).toEqual(mockResponse);
    expect(history.value).toContainEqual(mockResponse);
  });

  it("should handle error when wish fails", async () => {
    (invoke as any).mockRejectedValue(new Error("Test error"));

    const { doWish, loading, error } = useWish();

    await expect(doWish(1)).rejects.toThrow("Test error");

    expect(loading.value).toBe(false);
    expect(error.value).toContain("Test error");
  });
});