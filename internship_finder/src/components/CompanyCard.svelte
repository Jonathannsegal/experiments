<script>
  export let item;
  import { writable } from "svelte/store";
  import { tweened } from "svelte/motion";
  import { cubicOut } from "svelte/easing";
  import { onMount } from "svelte";

  const progress = tweened(0, {
    duration: 400,
    easing: cubicOut
  });

  onMount(async () => {
    progress.set(item.progress);
  });
</script>

<style>
  .cardContents {
    width: 100%;
    display: flex;
    align-items: center;
    justify-content: space-between;
  }
  .cardInfo {
    display: flex;
    flex-direction: column;
    justify-content: space-between;
  }
  .cardInfoGrid {
    display: flex;
    flex-direction: row;
    justify-content: space-between;
  }
  .bookmark {
    padding: 0;
    background-color: black;
    height: 2em;
    width: 1em;
  }
  .card {
    margin: 0 auto;
    width: 80%;
    height: 2.5em;
    /* border: 1px solid rgba(133, 133, 133, 0.2); */
    display: flex;
    /* justify-content: space-between; */
    /* display: flex;
    align-items: center;
    justify-content: center; */
  }
  .cardContentsInternal {
    display: flex;
    flex-direction: row;
  }

  .logoName {
    margin-left: 0.2em;
    width: 28%;
    display: flex;
    align-items: center;
  }
  .logo {
    max-height: 1em;
    max-width: 1em;
  }
  .companyName {
    margin-left: 0.3em;
    font-weight: 500;
    vertical-align: text-bottom;
  }

  .date {
    width: 20%;
    display: flex;
    align-items: center;
  }
  .dateApplied {
    width: 100%;
    text-align: right;
  }

  .progressBar {
    width: 30%;
    display: flex;
    align-items: center;
  }
  progress {
    display: block;
    width: 100%;
  }

  .location {
    width: 22%;
    display: flex;
    align-items: center;
  }
  .location * {
    width: 100%;
    text-align: left;
  }
</style>

<div class="card">
  <!-- <div class="cardContents"> -->

  <div class="logoName">
    <img class="logo" alt="" src={item.logo} />
    <span class="companyName">{item.company}</span>
  </div>

  <div class="location">
    <span>{item.location}</span>
  </div>

  <div class="progressBar">
    <progress value={$progress} />
  </div>

  <div class="date">
    <span class="dateApplied">{item.applyDate}</span>
  </div>

  <!-- <div class="cardContentsInternal">
    <div>
      <img class="logo" alt="" src={item.logo} />
    </div>
    <div>
      <p class="companyName">{item.company}</p>
    </div>
  </div> -->

  <!-- <div class="cardContentsInternal">
      <div class="cardInfo">

        <div>
            <h2 class="position">{item.position}</h2>
        </div>

        <div class="cardInfoGrid">
            <div>
                <p class="companyName">{item.company}</p>
                <p>alksjdlkasjdhi</p>
            </div>
            <div>
                <p>hlakjsdlkasji</p>
            </div>
        </div>

      </div>
    </div> -->

  <!-- <div class="cardContentsInternal">
        <div class="bookmark"/>
    </div>
  </div> -->
</div>
