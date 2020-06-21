<script>
  import VirtualList from "@sveltejs/svelte-virtual-list";
  import Hero from "../components/Hero.svelte";
  import CompanyCard from "../components/CompanyCard.svelte";
  let name = "Search list";

  const internships = [
    {
      company: "Google",
      logo:
        "https://upload.wikimedia.org/wikipedia/commons/5/53/Google_%22G%22_Logo.svg",
      location: "San Fransisco",
      positions: ["Software Engineer"],
      link: "https://careers.google.com/students/",
      applyDate: "Apr 18, 2020",
      progress: 0.2
    },
    {
      company: "Salesforce",
      logo:
        "https://assets.brandfolder.com/pyazhl-2y2gi8-246utq/v/6265116/original/Salesforce_Logo_Web_2019.png",
      progress: 0.5,
      applyDate: "Aug 20, 2020"
    },
    {
      company: "dwolla",
      logo: "",
      location: "San Fransisco",
      positions: ["Software Engineer"],
      link: "https://careers.google.com/students/",
      applyDate: "Apr 18, 2020",
      progress: 0.2
    },
    {
      company: "dwolla",
      logo: "",
      location: "San Fransisco",
      positions: ["Software Engineer"],
      link: "https://careers.google.com/students/",
      applyDate: "Apr 18, 2020",
      progress: 0.2
    },
    {
      company: "dwolla",
      logo: "",
      location: "San Fransisco",
      positions: ["Software Engineer"],
      link: "https://careers.google.com/students/",
      applyDate: "Apr 18, 2020",
      progress: 0.2
    }
  ];
</script>

<style>
  .container {
    background-color: white;
    width: 100%;
    height: 50%;
    /* height: 87.5%; */
  }
  .header {
    margin: 0 auto;
    width: 80%;
    height: 2.5em;
    display: flex;
    justify-content: space-between;
  }
  .name {
    margin-left: 0.2em;
    width: 28%;
    display: flex;
    align-items: center;
  }
  .progress {
    width: 30%;
    display: flex;
    align-items: center;
    text-align: left;
  }
  .date {
    width: 20%;
    display: flex;
    align-items: center;
  }
  .date * {
    width: 100%;
    text-align: right;
  }
  .headerItem {
    font-size: 0.8em;
    font-weight: 500;
    color: #8a92a0;
  }
  .button {
    width: max-content;
    background-color: #1f61dc; /* Green */
    border: none;
    color: white;
    font-size: 0.8em;
    border-radius: 8px;
    font-weight: 600;
    height: 32px;
    padding: 0 12px;
    text-align: center;
    text-decoration: none;
  }
  .controlls {
    margin: 1em auto;
    width: 80%;
    height: 2.5em;
    display: flex;
    justify-content: space-between;
  }
  .search {
    margin-left: 0.2em;
    width: 90%;
    display: flex;
    align-items: center;
  }
  .searchbar {
    width: 35%;
    text-indent: 1em;
    color: #d4d7dc;
    height: 30px;
    border: 1px solid #d4d7dc;
    border-radius: 8px;
  }
  .addButton {
    display: flex;
    align-items: center;
  }
  .location {
    width: 22%;
    display: flex;
    align-items: center;
    text-align: left;
  }
</style>

<svelte:head>
  <title>💼 Find an internship!</title>
</svelte:head>

<Hero />
<div class="container">

  <div class="controlls">
    <div class="search">
      <input class="searchbar" bind:value={name} />
    </div>
    <div class="addButton">
      <button class="button">Add a company</button>
    </div>
  </div>

  <div class="header">
    <div class="name">
      <span style="color: #3a74e1" class="headerItem">Name</span>
    </div>

    <div class="location">
      <span class="headerItem">location</span>
    </div>

    <div class="progress">
      <span class="headerItem">Application progress</span>
    </div>

    <div class="date">
      <span class="headerItem">Date Applied</span>
    </div>
  </div>

  <VirtualList items={internships} let:item>
    <CompanyCard {item} />
  </VirtualList>
</div>
