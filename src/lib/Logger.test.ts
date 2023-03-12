import { expect, test, describe } from "vitest";
import { fireEvent, render, screen } from "@testing-library/svelte";
import Logger from "./Logger.svelte";
import { TauriBackend } from "./tauri";
import { mock, instance, when } from "ts-mockito";
import "@testing-library/jest-dom";
import { tick } from "svelte";

describe("Logger", () => {
  const mockedTauri: TauriBackend = mock(TauriBackend);

  test("that the Logger is rendering data", async () => {
    when(mockedTauri.fetchContexts).thenReturn(() =>
      Promise.resolve(["ctx1", "ctx2", "ctx3"])
    );
    when(mockedTauri.fetchServices).thenReturn(() =>
      Promise.resolve(["svc1", "svc2", "svc3", "svc4"])
    );
    const tauriMock: TauriBackend = instance(mockedTauri);

    let rendered = render(Logger, { backend: tauriMock });
    await rendered.component.onMountHandle();

    // fireEvent.click(screen.getByTestId("logger-ctx"))

    expect(rendered.getByTestId("logger-ctx")).toBeInTheDocument();
    expect(rendered.getByTestId("logger-svc")).toBeInTheDocument();

    let { children: ctxOptions } = rendered.getByTestId("logger-ctx");
    expect(ctxOptions.length).toBe(3);
    expect(ctxOptions.item(0).innerHTML.trim()).toBe("ctx1");
    expect(ctxOptions.item(1).innerHTML.trim()).toBe("ctx2");
    expect(ctxOptions.item(2).innerHTML.trim()).toBe("ctx3");

    let { children: svcOptions } = rendered.getByTestId("logger-svc");
    expect(svcOptions.length).toBe(4);
    expect(svcOptions.item(0).innerHTML.trim()).toBe("svc1");
    expect(svcOptions.item(1).innerHTML.trim()).toBe("svc2");
    expect(svcOptions.item(2).innerHTML.trim()).toBe("svc3");
    expect(svcOptions.item(2).innerHTML.trim()).toBe("svc3");
  });
});
