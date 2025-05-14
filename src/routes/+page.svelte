<script lang="ts">
  import { goto } from '$app/navigation';
  import { onMount } from 'svelte';

  onMount(() => {
    document.body.style.margin = '0';
    document.body.style.padding = '0';
    document.body.style.overflow = 'hidden';
    document.documentElement.style.margin = '0';
    document.documentElement.style.padding = '0';
    document.documentElement.style.overflow = 'hidden';
  });
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

<div style="position: fixed; top: 0; left: 0; right: 0; bottom: 0; width: 100vw; height: 100vh; margin: 0; padding: 0; display: flex; flex-direction: column; align-items: center; justify-content: center; background-image: url('/blocktree-background.png'); background-size: cover; background-position: center; background-repeat: no-repeat; z-index: -2;">
  <img src="/blocktree-wallet-logo.png" alt="Blocktree Wallet Logo" style="width: 200px; height: auto; margin-bottom: 48px;" />

  <p style="max-width: 660px; color: #A6A6A6; font-family: Poppins, sans-serif; font-size: 16px; font-weight: 500; line-height: 24px; letter-spacing: 0.48px; text-align: left; margin-bottom: 96px;">
    Powering AI-driven economies and interstellar finance with Blocktree Wallet. The first tree-structure wallet.
  </p>

  <div style="display: flex; flex-direction: column; gap: 64px;">
    <div style="display: flex; align-items: center;">
      <p style="color: #FFF; font-family: Poppins, sans-serif; font-size: 16px; font-style: normal; font-weight: 500; line-height: 24px; letter-spacing: 0.48px; margin-right: 24px;">Already has a wallet?</p>
      <div style="position: relative; width: fit-content;">
        <div style="position: absolute; top: -1px; left: -1px; right: -1px; bottom: -1px; border-radius: 30px; background: linear-gradient(to right, #FFFFFF, #1D1D1D); z-index: 0;"></div>
        <button on:click={() => goto('/import')} style="position: relative; z-index: 1; border-radius: 30px; border: none; background: linear-gradient(270deg, #212430 0%, #717171 100%); color: #FFF; font-family: 'Poppins', sans-serif; font-size: 16px; font-weight: 600; line-height: 24px; letter-spacing: 0.48px; padding: 10px 30px; margin: 1px;">
          Import Wallet
        </button>
      </div>
    </div>
    
    <div style="display: flex; align-items: center;">
      <p style="color: #FFF; font-family: Poppins, sans-serif; font-size: 16px; font-style: normal; font-weight: 500; line-height: 24px; letter-spacing: 0.48px; margin-right: 24px;">Want a new wallet?</p>
      <div style="position: relative; width: fit-content;">
        <div style="position: absolute; top: -1px; left: -1px; right: -1px; bottom: -1px; border-radius: 30px; background: linear-gradient(to right, #FFFFFF, #1D1D1D); z-index: 0;"></div>
        <button on:click={() => goto('/create')} style="position: relative; z-index: 1; border-radius: 30px; border: none; background: linear-gradient(270deg, #212430 0%, #717171 100%); color: #FFF; font-family: 'Poppins', sans-serif; font-size: 16px; font-weight: 600; line-height: 24px; letter-spacing: 0.48px; padding: 10px 30px; margin: 1px;">
          Create New Wallet
        </button>
      </div>
    </div>
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
</style>