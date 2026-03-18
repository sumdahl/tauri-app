<script lang="ts">
  import { greet, getAppInfo, initializeApp } from "$lib/api";
  import Button from "$lib/components/ui/button/button.svelte";
  let name = $state("");
  let greetMsg = $state("");
  let greetingCount = $state(0);
  let isLoading = $state(false);
  async function handleGreet(event: Event) {
    event.preventDefault();
    if (!name.trim()) return;
    
    isLoading = true;
    try {
      const response = await greet(name);
      greetMsg = response.message;
      greetingCount = response.greeting_count;
    } catch (error) {
      console.error("Failed to greet:", error);
    } finally {
      isLoading = false;
    }
  }
</script>
<main class="min-h-screen bg-gray-50 flex items-center justify-center px-4">
  <div class="w-full max-w-md">
    <div class="bg-white rounded-xl shadow-sm border border-gray-200 p-8 mb-6">
      <div class="text-center">
        <div class="w-16 h-16 bg-primary-100 rounded-full flex items-center justify-center mx-auto mb-4">
          <svg class="w-8 h-8 text-primary-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" />
          </svg>
        </div>
        <h1 class="text-2xl font-semibold text-gray-900 mb-2">Welcome to Tauri</h1>
        <p class="text-gray-500 text-sm">Build cross-platform desktop apps with web technologies</p>
      </div>
    </div>
    <div class="bg-white rounded-xl shadow-sm border border-gray-200 p-8">
      <h2 class="text-lg font-medium text-gray-900 mb-6">Get Started</h2>
      
      <form onsubmit={handleGreet} class="space-y-4">
        <div>
          <label for="greet-input" class="block text-sm font-medium text-gray-700 mb-2">
            Your Name
          </label>
          <input 
            id="greet-input"
            type="text"
            class="w-full px-4 py-3 bg-gray-50 border border-gray-200 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-transparent transition-all duration-200"
            placeholder="Enter your name..."
            bind:value={name}
            disabled={isLoading}
          />
        </div>
        <Button
          type="submit"
          disabled={isLoading || !name.trim()}
        >
          {#if isLoading}
            <span class="flex items-center justify-center gap-2">
              <svg class="animate-spin h-5 w-5" fill="none" viewBox="0 0 24 24">
                <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
              </svg>
              Processing...
            </span>
          {:else}
            Greet Me
          {/if}
          </Button>
      </form>
      {#if greetMsg}
        <div class="mt-6 p-4 bg-primary-50 border border-primary-100 rounded-lg">
          <p class="text-primary-700 text-sm font-medium">{greetMsg}</p>
          <p class="text-primary-600 text-xs mt-2">Total greetings: {greetingCount}</p>
        </div>
      {/if}
    </div>
  </div>
</main>
