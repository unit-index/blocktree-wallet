<script lang="ts">
  import { goto } from '$app/navigation';
  import { onMount } from 'svelte';
  import { validateMnemonic } from 'bip39';

  let phraseWords = Array(12).fill('');
  let focusedIndex: number | null = null;
  let errorMessage: string | null = null;

  onMount(() => {
    document.body.style.margin = '0';
    document.body.style.padding = '0';
    document.body.style.overflow = 'hidden';
    document.documentElement.style.margin = '0';
    document.documentElement.style.padding = '0';
    document.documentElement.style.overflow = 'hidden';
  });

  function handleInput(index: number, event: Event) {
    const input = event.target as HTMLInputElement;
    phraseWords[index] = input.value.trim();
    phraseWords = [...phraseWords];
    errorMessage = null;
  }

  function handleFocus(index: number) {
    focusedIndex = index;
  }

  function handleBlur() {
    focusedIndex = null;
  }

  function validatePhrase(): boolean {
    const allFilled = phraseWords.every(word => word.length > 0);
    if (!allFilled) {
      return false;
    }
    const phrase = phraseWords.map(word => word.toLowerCase()).join(' ');
    try {
      return validateMnemonic(phrase);
    } catch (e) {
      return false;
    }
  }

  function handleEnter() {
    if (validatePhrase()) {
      console.log('Entered phrase:', phraseWords.join(' '));
      goto('/wallet');
    } else {
      errorMessage = 'Invalid recovery phrase.';
    }
  }

  function handleBack() {
    goto('/');
  }
</script>

<svelte:head>
  <style>
    :global(*) {
      margin: 0;
      padding: 0;
      box-sizing: border-box;
    }
    
    :global(html), :global(body) {
      margin: 0 !important;
      padding: 0 !important;
      overflow: hidden !important;
      width: 100% !important;
      height: 100% !important;
      position: fixed !important;
      top: 0 !important;
      left: 0 !important;
      right: 0 !important;
      bottom: 0 !important;
    }
  </style>
</svelte:head>

<div
  style="
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    width: 100vw;
    height: 100vh;
    margin: 0;
    padding: 0;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    background-color: #0C0B0B;
    z-index: -2;
  "
>
  <!-- Top Background Image -->
  <img
    src="/pre-page-background.png"
    alt="Pre-Page Background"
    style="
      position: absolute;
      top: 0;
      left: 0;
      width: 100%;
      height: auto;
      z-index: -1;
    "
  />

  <!-- Logo -->
  <img
    src="/blocktree-logo.png"
    alt="Blocktree Wallet Logo"
    style="width: 100px; height: auto; margin-bottom: 32px;"
  />

  <!-- Title -->
  <h1
    style="
      color: #FFF;
      font-family: Poppins, sans-serif;
      font-size: 40px;
      font-style: normal;
      font-weight: 500;
      line-height: 60px;
      letter-spacing: 1.2px;
      text-align: center;
      margin-bottom: 16px;
    "
  >
    Import Wallet
  </h1>

  <!-- Description -->
  <p
    style="
      color: #707070;
      font-family: Poppins, sans-serif;
      font-size: 16px;
      font-style: normal;
      font-weight: 500;
      line-height: 24px;
      letter-spacing: 0.48px;
      text-align: center;
      margin-bottom: 64px;
    "
  >
    Type your 12-Word Phrase
  </p>

  <!-- Input Grid (6 columns x 2 rows) -->
  <div
    style="
      display: grid;
      grid-template-columns: repeat(6, 1fr);
      grid-template-rows: repeat(2, auto);
      gap: 48px;
      margin-bottom: 32px;
      max-width: 100%;
      padding: 0 16px;
    "
  >
    {#each Array(12) as _, index}
      <div style="display: flex; flex-direction: column; align-items: center;">
        <!-- Number -->
        <span
          style="
            color: {focusedIndex === index ? '#FFFFFF' : '#707070'};
            font-family: Poppins, sans-serif;
            font-size: 20px;
            font-style: normal;
            font-weight: 500;
            line-height: 24px;
            letter-spacing: 0.6px;
            margin-bottom: 0px;
            transition: color 0.3s ease;
          "
        >
          {index + 1}
        </span>
        <!-- Input Line -->
        <input
          type="text"
          value={phraseWords[index]}
          on:input={(e) => handleInput(index, e)}
          on:focus={() => handleFocus(index)}
          on:blur={handleBlur}
          class="input-animate"
          style="
            width: 100%;
            max-width: 120px;
            border: none;
            border-bottom: 2px solid {focusedIndex === index ? '#FFFFFF' : '#707070'};
            background: transparent;
            color: {focusedIndex === index ? '#FFFFFF' : '#707070'};
            font-family: Poppins, sans-serif;
            font-size: 16px;
            font-weight: 500;
            text-align: center;
            padding: 4px 0;
            outline: none;
            transition: all 0.3s ease;
          "
        />
      </div>
    {/each}
  </div>

  <!-- Error Message -->
  {#if errorMessage}
    <p
      class="error-message"
      style="
        color: #963D26;
        font-family: Poppins, sans-serif;
        font-size: 16px;
        font-style: normal;
        font-weight: 500;
        line-height: 24px;
        letter-spacing: 0.48px;
        text-align: center;
        margin-bottom: 32px;
      "
    >
      {errorMessage}
    </p>
  {/if}

  <!-- Enter Button -->
  <button
    on:click={handleEnter}
    class="enter-button"
    style="
      border-radius: 20px;
      background: #1E1E1E;
      padding: 10px 20px;
      margin-bottom: 64px;
      border: none;
      transition: transform 0.2s ease, background 0.3s ease;
    "
  >
    <span
      style="
        color: #FFF;
        font-family: Poppins, sans-serif;
        font-size: 16px;
        font-style: normal;
        font-weight: 500;
        line-height: 24px;
        letter-spacing: 0.48px;
      "
    >
      Enter
    </span>
  </button>

  <!-- Bottom Navigation (Back and Help) -->
  <div
    style="
      display: flex;
      justify-content: space-between;
      align-items: center;
      width: 100%;
      padding: 0 200px;
      position: absolute;
      bottom: 32px;
    "
  >
    <!-- Back -->
    <button
      on:click={handleBack}
      class="nav-button"
      style="
        display: flex;
        align-items: center;
        background: none;
        border: none;
        cursor: pointer;
        transition: transform 0.2s ease;
      "
    >
      <img
        src="/back-arrow.png"
        alt="Back Arrow"
        style="width: 5px; height: 8px; margin-right: 8px; transition: transform 0.2s ease;"
      />
      <span
        style="
          color: #707070;
          font-family: Poppins, sans-serif;
          font-size: 16px;
          font-style: normal;
          font-weight: 400;
          line-height: 24px;
          letter-spacing: 0.48px;
          transition: color 0.3s ease;
        "
      >
        Back
      </span>
    </button>

    <!-- Help -->
    <button
      class="nav-button"
      style="
        display: flex;
        align-items: center;
        background: none;
        border: none;
        cursor: pointer;
        transition: transform 0.2s ease;
      "
    >
      <img
        src="/help.png"
        alt="Help Icon"
        style="width: 11px; height: 11px; margin-right: 4px; transition: transform 0.2s ease;"
      />
      <span
        style="
          color: #707070;
          font-family: Poppins, sans-serif;
          font-size: 16px;
          font-style: normal;
          font-weight: 400;
          line-height: 24px;
          letter-spacing: 0.48px;
          transition: color 0.3s ease;
        "
      >
        Help
      </span>
    </button>
  </div>
</div>

<style>
  @font-face {
    font-family: 'Poppins';
    src: url('/fonts/Poppins-Regular.ttf') format('truetype');
    font-weight: 500;
    font-style: normal;
  }
  @font-face {
    font-family: 'Poppins';
    src: url('/fonts/Poppins-Bold.ttf') format('truetype');
    font-weight: 700;
    font-style: normal;
  }

  :global(body), :global(html) {
    margin: 0;
    padding: 0;
    overflow: hidden;
    width: 100%;
    height: 100%;
    position: fixed;
  }

  :global(*) {
    box-sizing: border-box;
  }

  .input-animate:focus {
    transform: scale(1.05); 
    animation: shake 0.3s ease; 
  }

  @keyframes shake {
    0%, 100% { transform: translateX(0); }
    25% { transform: translateX(-2px); }
    75% { transform: translateX(2px); }
  }

  .error-message {
    animation: fadeIn 0.3s ease-in, shake 0.5s ease;
  }

  @keyframes fadeIn {
    from { opacity: 0; }
    to { opacity: 1; }
  }

  .enter-button:hover {
    transform: scale(1.05);
    background: #2A2A2A;
  }

  .enter-button:active {
    transform: scale(0.95); 
  }

  .enter-button.error {
    animation: shake 0.5s ease;
  }

  .nav-button:hover img {
    transform: translateX(-3px);
  }

  .nav-button:active {
    transform: scale(0.95);
  }
</style>