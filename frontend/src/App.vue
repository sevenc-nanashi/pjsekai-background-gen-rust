<script setup lang="ts">
import renderWorker from "./worker?worker";
import { onMounted, ref } from "vue";
import defaultV1 from "./defaults/v1.png";
import defaultV3 from "./defaults/v3.png";
import { v4 as uuidV4 } from "uuid";

const upload = ref<HTMLInputElement | undefined>(undefined);
const resultV1 = ref<HTMLImageElement | undefined>(undefined);
const resultV3 = ref<HTMLImageElement | undefined>(undefined);

const generating = ref(false);
let worker: Worker | undefined;

const callbacks: Record<
  string,
  (data: { v1: Uint8Array; v3: Uint8Array }) => void
> = {};

onMounted(() => {
  worker = new renderWorker();
  worker.onmessage = (e) => {
    const { nonce, data } = e.data;
    console.log(`[web] -> ${nonce}`);
    if (!callbacks[nonce]) {
      console.warn("Unknown nonce");
      return;
    }

    callbacks[nonce](data);
    delete callbacks[nonce];
  };
});

const getAverageRGB = async (image: HTMLImageElement) => {
  const imageCanvas = document.createElement("canvas");
  imageCanvas.width = 1;
  imageCanvas.height = 1;

  const imageCtx = imageCanvas.getContext("2d");
  if (!imageCtx) {
    throw new Error("Failed to get canvas context");
  }

  imageCtx.drawImage(image, 0, 0, 1, 1);
  const imageData = imageCtx.getImageData(0, 0, 1, 1).data;
  return {
    r: imageData[0],
    g: imageData[1],
    b: imageData[2],
  };
};

const resizeImage = async (image: HTMLImageElement) => {
  const average = await getAverageRGB(image);
  const imageCanvas = document.createElement("canvas");
  imageCanvas.width = 512;
  imageCanvas.height = 512;

  const imageCtx = imageCanvas.getContext("2d");
  if (!imageCtx) {
    throw new Error("Failed to get canvas context");
  }

  imageCtx.fillStyle = `rgb(${average.r}, ${average.g}, ${average.b})`;
  imageCtx.fillRect(0, 0, 512, 512);
  let newWidth: number, newHeight: number;
  if (image.width > image.height) {
    newWidth = 512;
    newHeight = Math.round((image.height / image.width) * 512);
  } else {
    newWidth = Math.round((image.width / image.height) * 512);
    newHeight = 512;
  }

  imageCtx.drawImage(
    image,
    0,
    0,
    image.width,
    image.height,
    (512 - newWidth) / 2,
    (512 - newHeight) / 2,
    newWidth,
    newHeight
  );

  return imageCanvas;
};

const generate = async () => {
  const file = upload.value?.files?.[0];

  if (!file) {
    return;
  }
  if (!resultV1.value || !resultV3.value) {
    throw new Error("[web] Failed to render");
  }

  const image = new Image();
  image.src = URL.createObjectURL(file);

  await image.decode();

  const resizedImage = await resizeImage(image);

  const basePng = await new Promise<Blob>((resolve, reject) => {
    resizedImage.toBlob((blob) => {
      if (!blob) {
        reject(new Error("Failed to convert canvas to blob"));
        return;
      }
      resolve(blob);
    }, "image/png");
  });
  const arrayBuffer = await basePng.arrayBuffer();

  const nonce = uuidV4();

  generating.value = true;
  const result = await new Promise<{ v1: Uint8Array; v3: Uint8Array }>(
    (resolve) => {
      console.log(`[web] ${nonce} ->`);
      worker?.postMessage(
        {
          nonce,
          data: arrayBuffer,
        },
        [arrayBuffer]
      );
      callbacks[nonce] = resolve;
    }
  );
  generating.value = false;

  resultV1.value.src = URL.createObjectURL(
    new Blob([result.v1], { type: "image/png" })
  );
  resultV3.value.src = URL.createObjectURL(
    new Blob([result.v3], { type: "image/png" })
  );
};

const download = (src: string | undefined, name: string) => {
  if (!src) {
    return;
  }

  const a = document.createElement("a");
  a.href = src;
  a.download = name;
  a.style.display = "none";

  document.body.appendChild(a);
  a.click();
  document.body.removeChild(a);
};
</script>

<template>
  <header
    class="bg-cyan-400 dark:bg-slate-950 text-xl font-bold p-4 text-white dark:text-cyan-200"
  >
    PJSekai Background Generator: Web
  </header>

  <div class="md:w-3/4 md:mx-auto mx-4 my-4 flex flex-col gap-4">
    <div class="w-full flex flex-wrap md:flex-row flex-col gap-2">
      <label class="align-middle cursor-pointer">
        <span class="my-auto"> 画像をアップロードしてください。 </span>
        <button
          class="bg-cyan-400 hover:bg-cyan-500 text-white font-bold py-1 px-4 rounded ml-4"
          @click="() => upload?.click()"
        >
          画像を選択
        </button>
      </label>
      <div class="my-auto">
        <span class="text-gray-500 md:ml-4">ダウンロード</span>
        <button
          class="bg-cyan-400 hover:bg-cyan-500 text-white font-bold py-1 px-4 rounded ml-4"
          @click="() => download(resultV3?.src, 'v3.png')"
        >
          新背景
        </button>
        <button
          class="bg-cyan-400 hover:bg-cyan-500 text-white font-bold py-1 px-4 rounded ml-2"
          @click="() => download(resultV1?.src, 'v1.png')"
        >
          旧背景
        </button>
      </div>
      <input
        class="hidden"
        type="file"
        accept="image/*"
        ref="upload"
        @change="generate()"
      />
    </div>
    <div
      class="w-full gap-4 md:flex-row flex flex-col transition-opacity"
      :style="{
        opacity: generating ? 0.5 : 1,
        cursor: generating ? 'wait' : 'default',
      }"
    >
      <div class="md:w-1/2">
        <img ref="resultV3" :src="defaultV3" />
      </div>
      <div class="md:w-1/2">
        <img ref="resultV1" :src="defaultV1" />
      </div>
    </div>
  </div>
</template>
