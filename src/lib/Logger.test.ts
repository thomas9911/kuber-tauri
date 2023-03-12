import { expect, test, describe } from "vitest";
import { fireEvent, render, screen } from "@testing-library/svelte";
import Logger from "./Logger.svelte";
import { TauriBackend } from "./tauri";
import { mock, instance, when } from "ts-mockito";
import "@testing-library/jest-dom";
import { tick } from "svelte";

describe("Logger", () => {
  const mockedTauri: TauriBackend = mock(TauriBackend);
  when(mockedTauri.fetchContexts).thenReturn(() =>
    Promise.resolve(["ctx1", "ctx2", "ctx3"])
  );
  when(mockedTauri.fetchServices).thenReturn(() =>
    Promise.resolve(["svc1", "svc2", "svc3", "svc4"])
  );
  when(mockedTauri.cancelMessages).thenReturn(() => Promise.resolve());
  when(mockedTauri.setCtx).thenReturn(() => Promise.resolve());
  when(mockedTauri.setSvc).thenReturn(() => Promise.resolve());

  test("is loading data on mount", async () => {
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
    expect(svcOptions.item(3).innerHTML.trim()).toBe("svc4");

    expect(rendered.component.introspectState()).toMatchObject({
      ctxs: ["ctx1", "ctx2", "ctx3"],
      svcs: ["svc1", "svc2", "svc3", "svc4"],
    });
  });

  test("switches to selected context", async () => {
    const tauriMock: TauriBackend = instance(mockedTauri);

    let rendered = render(Logger, { backend: tauriMock });
    await rendered.component.onMountHandle();

    expect(rendered.getByTestId("logger-ctx")).toBeInTheDocument();
    expect(rendered.getByTestId("logger-svc")).toBeInTheDocument();

    fireEvent.change(rendered.getByTestId("logger-ctx"), {
      target: { value: "ctx2" },
    });
    fireEvent.change(rendered.getByTestId("logger-svc"), {
      target: { value: "svc4" },
    });

    expect(rendered.component.introspectState()).toMatchObject({
      selectedCtx: "ctx2",
      selectedSvc: "svc4",
    });
  });
});
