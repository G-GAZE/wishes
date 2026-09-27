import { invoke } from "@tauri-apps/api/core";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { useWish } from "../useWish";


vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(),
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

  it("should handle successful ten wish via batch command", async () => {
    const mockResults = Array.from({ length: 10 }, (_, i) => ({
      id: i + 1,
      content: `Test-Card-${i}`,
      tags: [],
      event_tags: [],
    }));
    (invoke as any).mockResolvedValue(mockResults);

    const { doWishTen, loading, history } = useWish();
    const results = await doWishTen(1);

    expect(invoke).toHaveBeenCalledWith("wish_ten", { bannerId: 1 });
    expect(results).toHaveLength(10);
    expect(loading.value).toBe(false);
    expect(history.value[0]).toEqual(mockResults[0]);
  });
});