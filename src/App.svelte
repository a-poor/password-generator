<script lang="ts">
  import "./app.css"
</script>

<main class="w-full h-full">
  <div class="my-5 mx-6 font-mono">
    <h1 class="text-3xl font-bold mb-1">Password Generator</h1>
    <p class="text-md mb-7">
      Generate passwords with ease
    </p>

    <p class="text-md mb-1">
      Your password:
    </p>
    <div class="flex bg-zinc-500 rounded-lg w-60 mb-5">
      <span class="p-2 flex-grow">
        super-secret-value
      </span>
      <button class="bg-cyan-800 rounded">
        <img src="/copy-icon.svg" alt="Copy Icon" class="w-5 h-5 stroke-5"/>
      </button>
    </div>

    <form class="text-sm">
      <div class="block mb-4">
        <label for="num-words" class="mb-2">Number of words</label>
        <input
          type="number"
          id="num-words"
          name="num-words"
          value="4"
          class="border-2 border-gray-300 p-2 rounded-md block mt-1"
        />
      </div>
      <div class="block mb-4">
        <label for="num-words" class="mb-2">Separator</label>
        <input
          type="text"
          id="separator"
          name="separator"
          value="-"
          class="border-2 border-gray-300 p-2 rounded-md block mt-1"
        />
      </div>
    </form>
  </div>
</main>

<style>
</style>