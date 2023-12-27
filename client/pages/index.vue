<script setup>
import { ref, reactive, getCurrentInstance } from "vue";
import init, { calc } from '~/wasm/pkg/wasm.js';

let text = ref('');

let append = (char) => {
  text.value = text.value.concat(char);
  console.log(text.value);
}

let icons = [
  '😀', '😃', '😄', '😁', '😆', '😅', '😂', '🤣', 
  '🥲', '🥹', '☺️', '😊', '😇', '🙂', '🙃', '😉', 
  '😌', '😍', '🥰', '😘', '😗', '😙', '😚', '😋', 
  '😛', '😝', '😜', '🤪', '🤨', '🧐', '🤓', '😎', 
  '🥸', '🤩', '🥳', '😏', '😒', '😞', '😔', '😟', 
  '😕', '🙁', '☹️', '😣', '😖', '😫', '😩', '🥺', 
  '😢', '😭', '😮‍💨', '😤', '😠', '😡', '🤬', '🤯', 
  '😳', '🥵', '🥶', '😱', '😨', '😰', '😥', '😓', 
  '🫣', '🤗', '🫡', '🤔', '🫢', '🤭', '🤫', '🤥', 
  '😶', '😶‍🌫️', '😐', '😑', '😬', '🫨', '🫠', '🙄', 
  '😯', '😦', '😧', '😮', '😲', '🥱', '😴', '🤤', 
  '😪', '😵', '😵‍💫', '🫥', '🤐', '🥴', '🤢', '🤮', 
  '🤧', '😷', '🤒', '🤕', '🤑', '🤠', '😈', '👿', 
  '👹', '👺', '🤡', '💩', '👻', '💀', '☠️', '👽', 
  '👾', '🤖', '🎃', '😺', '😸', '😹', '😻', '😼', 
  '😽', '🙀', '😿', '😾', 
];

let wait = (ms) => new Promise(resolve => setTimeout(resolve, ms));

let num = ref(1000000000);

let rs = reactive({
  value: 0,
  formatted: computed(() => rs.value.toString().replace(/\B(?=(\d{3})+(?!\d))/g, ",")),
  ellipse: 0,
  loading: false,
});

let js = reactive({
  value: 0,
  formatted: computed(() => js.value.toString().replace(/\B(?=(\d{3})+(?!\d))/g, ",")),
  ellipse: 0,
  loading: false,
});

onMounted(() => {
  init();
});

async function startCalc() {
  if(!num.value) {
    alert("num > 0 !");
    return;
  }

  clearCalc();

  rs.loading = true;
  await wait(30);

  let rsStart = performance.now();
  rs.value = calc(BigInt(num.value));
  rs.ellipse = Math.round(performance.now() - rsStart);
  rs.loading = false;
  await wait(30);

  js.loading = true;
  await wait(30);

  let jsStart = performance.now();
  let i = 0;
  while(i < num.value) {
    i++;
  }
  js.value = i;
  js.ellipse = Math.round(performance.now() - jsStart);
  js.loading = false;
}

function clearCalc() {
  rs.value = 0;
  rs.ellipse = 0;
  js.value = 0;
  js.ellipse = 0;
}
</script>

<template>
  <article class="container">
    <article class="interaction">
      <textarea v-model="text"/>
      <HeadlessPopover class="relative">
        <HeadlessPopoverButton class="bg-green-200 rounded-md px-2 py-1">
          {{ icons[0] }}
        </HeadlessPopoverButton>

        <HeadlessPopoverPanel class="absolute z-10 bg-slate-50 mt-3 px-2 py-2">
          <div class="grid grid-cols-8 gap-x-4 gap-y-1">
            <div class="no-drag cursor-pointer" v-for="icon of icons" @click="append(icon)">
              {{ icon }}
            </div>
          </div>
        </HeadlessPopoverPanel>
      </HeadlessPopover>
    </article>
    <article class="calculation">
      <div class="control">
        <span>{{ `for (let i = 0; i <` }}</span>
        <input type="number" v-model="num" :step="100000000"/>
        <span>{{  `; i++) {` }}</span>
        <button @click="() => startCalc()" :disabled="rs.loading || js.loading">start</button>
        <button @click="() => clearCalc()" :disabled="rs.loading || js.loading">clear</button>
      </div>
      <div class="result">
        <div>{{ `Rust(Wasm) :` }}</div>
        <div v-if="rs.loading">⋯</div>
        <div v-else>{{ `${rs.formatted} (${rs.ellipse}ms)` }}</div>
      </div>
      <div class="result">
        <div>{{ `Javascript    :` }}</div>
        <div v-if="js.loading">⋯</div>
        <div v-else>{{ `${js.formatted} (${js.ellipse}ms)` }}</div>
      </div>
      <div>
        <span>{{ `}` }}</span>
      </div>
    </article>
  </article>
</template>

<style scoped lang="scss">
$gray-400: #9ca3af;
article.interaction {
  margin-top: 4rem;

  >textarea {
    width: 100%;
    padding: 0.25rem;
    border: 1px solid $gray-400;
    border-radius: 0.25rem;

    &:focus, &:focus-visible {
        border: 1px solid blue;
        box-shadow: none;
        outline: none;
      }
  }
}

article.calculation {
  margin-top: 4rem;

  * {
    white-space: pre;
  }

  div.control {
    display: flex;
    flex-flow: row nowrap;
    gap: 0.5rem;
    align-items: center;
    
    input {
      border: 1px solid $gray-400;
      border-radius: 0.25rem;
      padding: 0.125rem 0 0.125rem 0.25rem;

      &:focus, &:focus-visible {
        border: 1px solid blue;
        box-shadow: none;
        outline: none;
      }
    }
    button {
      padding: 0.25rem 0.5rem;
      background-color: beige;
      border-radius: 0.25rem;

      &:disabled {
        background-color: $gray-400;
      }
    }
  }
  div.result {
    margin-left: 2rem;
    display: flex;
    flex-flow: row nowrap;
    gap: 0.125rem;
  }
}
</style>