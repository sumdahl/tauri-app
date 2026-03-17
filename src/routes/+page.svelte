<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  let name = $state("");
  let greetMsg = $state("");

  async function greet(event: Event) {
    event.preventDefault();
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    greetMsg = await invoke("greet", { name });
  }
</script>


<main class="container mx-auto px-4 py-10 flex flex-col items-center">
  <h1 class="text-3xl font-bold mb-8">Welcome to Tauri + Svelte</h1>
  
  <div class="flex gap-4">
    <a href="https://vite.dev" class="text-blue-500 hover:text-blue-700">
      Vite
    </a>
  </div>
  
  <form class="flex gap-2 mt-4" onsubmit={greet}>
    <input 
      id="greet-input" 
      class="px-4 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
      placeholder="Enter a name..."
      bind:value={name} 
    />
    <button 
      type="submit"
      class="px-4 py-2 bg-blue-500 text-white rounded-lg hover:bg-blue-600"
    >
      Greet
    </button>
  </form>
  <p>{greetMsg}</p>
</main>

