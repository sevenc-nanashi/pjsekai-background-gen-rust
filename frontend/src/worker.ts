import * as bg from "pjsekai-background-gen-wasm";

self.addEventListener(
  "message",
  (event: MessageEvent<{ nonce: string; data: Uint8Array }>) => {
    console.log(`[worker] -> ${event.data.nonce}`);
    const message = event.data;
    const rsResultV1 = bg.renderV1(new bg.Image(new Uint8Array(message.data)));
    const rsResultV3 = bg.renderV3(new bg.Image(new Uint8Array(message.data)));

    console.log(`[worker] ${event.data.nonce} ->`);
    self.postMessage({
      nonce: message.nonce,
      data: {
        v1: rsResultV1.data,
        v3: rsResultV3.data,
      },
    });
  }
);

export default {};
