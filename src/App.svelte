<script lang="ts">
  import { writable, get } from 'svelte/store';

  import "./app.css"
  import CopyIcon from "./lib/CopyIcon.svelte";
  import ResetIcon from "./lib/ResetIcon.svelte";
  import GenerateIcon from './lib/GenerateIcon.svelte';

  import { invoke } from '@tauri-apps/api/tauri';


  const defaultFormValues = {
    nwords: 4,
    separator: "-",
  };

  let password: string | undefined;
  let passhash: string | undefined;

  const formData = writable({
    nwords: defaultFormValues.nwords,
    separator: defaultFormValues.separator,
  });


  function onReset() {
    formData.set(defaultFormValues);
    password = undefined;
    passhash = undefined;
  }

  async function onGenerate() {
    // Log that it was generated...
    console.log("Calling on-generate...");

    // Extract the form data
    const { nwords, separator } = get(formData);
    console.log("nwords: ", nwords, "separator: ", separator);

    try {
      // Call the backend generator
      const data: {password: string, passhash: string} = await invoke("generate_password", {
        nwords,
        separator,
      });

      console.log("Returned successfully: ", data);

      // Set the password value
      password = data.password;
      passhash = data.passhash;
    
    } catch (err) {
      // Log any error that occurs
      console.error("Failed to generate password:", err);
    }
  }

  function onCopy(data: string) {
    navigator.clipboard.writeText(data);
  }

  function onCopyPassword() {
    onCopy(password);
    document.querySelector("#generated-password")
  }

  function onCopyPassHash() {
    onCopy(passhash);
  }

</script>

<main class="absolute w-full h-full bg-slate-800 text-slate-50 uppercase">
  <!-- Page Header -->
  <div class="my-5 mx-6 font-mono">
    <h1 class="text-3xl font-bold mb-1">Password Generator</h1>
    <p class="text-md mb-7 text-slate-500">
      Generate secure random passwords with ease!
    </p>

    <!-- Password Display -->
    <p class="text-md mb-2">
      Password
    </p>
    <div class="flex bg-slate-700 rounded-xl w-96 mb-5 overflow-hidden">
      <span class="px-3 py-2 flex-grow select-all normal-case truncate">
        {#if password}
          <span id="generated-password">
            {password}
          </span>
        {:else}
          <span class="text-slate-400 select-none">
            super-secret-password
          </span>
        {/if}
      </span>
      <button 
        on:click={onCopyPassword}
        class="py-2 px-1 bg-sky-500 hover:bg-sky-400 active:bg-sky-600"
      >
        <CopyIcon />
      </button>
    </div>

    <p class="text-md mb-2">
      Bcrypt Hash
    </p>
    <div class="flex bg-slate-700 rounded-xl w-96 mb-5 overflow-hidden">
      <span class="px-3 py-2 flex-grow select-all normal-case truncate">
        {#if passhash}
          <span id="generated-hash">
            { passhash }
          </span>
        {:else}
          <span class="text-slate-400 select-none">
            your-passwords-hash
          </span>
        {/if}
      </span>
      <button 
        on:click={onCopyPassHash}
        class="py-2 px-1 bg-sky-500 hover:bg-sky-400 active:bg-sky-600"
      >
        <CopyIcon />
      </button>
    </div>

    <br/>

    <!-- Page Header -->
    <form class="text-sm">
      <div class="block mb-4">
        <label 
          for="num-words" 
          class="mb-2"
        >
          Number of words
        </label>
        <input
          type="number"
          name="num-words"
          min="1"
          max="6"
          bind:value={$formData.nwords}
          class="border-gray-300 p-2 rounded-lg block mt-1 bg-slate-700 w-48"
        />
      </div>
      <div class="block mb-4">
        <label 
          for="num-words" 
          class="mb-2"
        >
          Separator
        </label>
        <input
          type="text"
          name="separator"
          maxlength="1"
          bind:value={$formData.separator}
          class="border-gray-300 p-2 rounded-lg block mt-1 bg-slate-700 w-48"
        />
      </div>

      <!-- Action Buttons -->
      <div class="mt-7 flex space-x-3">
        <button 
          type="button" 
          class="flex items-center p-2 bg-sky-500 hover:bg-sky-400 active:bg-sky-600 rounded-xl"
          on:click={async () => { 
            await onGenerate();
          }}
        >
          <span class="mr-2 align-bottom">
            <GenerateIcon />
          </span>
          <span class="uppercase">
            Generate
          </span>
        </button>

        <button 
          type="button" 
          class="flex items-center p-2 bg-slate-500 hover:bg-slate-400 active:bg-slate-600 rounded-xl"
          on:click={onReset}
        >
          <span class="mr-2 align-bottom">
            <ResetIcon />
          </span>
          <span class="uppercase">
            Reset
          </span>
        </button>
      </div>
    </form>
  </div>
</main>

<style>
</style>