import type * as NodeCryptoModule from "node:crypto";
import * as ReactNavigationNativeModule from "@react-navigation/native";
import { renderHook, waitFor } from "@testing-library/react";
import * as ExpoCryptoModule from "expo-crypto";
import { StoreContextProvider } from "@/components/StoreContext";
import { useTodayScreen } from "@/components/TodayScreen/hooks/useTodayScreen";

jest.mock("@react-navigation/native", () => ({
  useNavigation: jest.fn(),
}));

describe("useTodayScreen", () => {
  beforeEach(() => {
    jest.useFakeTimers();

    jest.spyOn(ExpoCryptoModule, "randomUUID").mockImplementation(() => {
      const { randomUUID } =
        jest.requireActual<typeof NodeCryptoModule>("node:crypto");
      return randomUUID();
    });
  });

  afterEach(() => {
    jest.useRealTimers();
    jest.clearAllMocks();
  });

  test("headerTitle", async () => {
    const now = new Date("2020-01-02T03:04:05.678Z");
    jest.setSystemTime(now);
    const setOptions = jest.fn();
    jest.spyOn(ReactNavigationNativeModule, "useNavigation").mockReturnValue({
      setOptions,
    });

    renderHook(() => useTodayScreen(), {
      wrapper: ({ children }) => (
        <StoreContextProvider>{children}</StoreContextProvider>
      ),
    });

    await waitFor(() => {
      expect(setOptions).toHaveBeenCalledTimes(1);
    });
    expect(setOptions).toHaveBeenCalledWith({
      headerTitle: "Today 2020-01-02",
    });
  });
});
