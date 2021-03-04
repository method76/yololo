<template>

  <div id="app">

    <div class="header">
      <div class="pure-menu pure-menu-horizontal">
        <div style="float: left; padding-left: 12px;"><img src="./assets/img/ic-logo.png" style="height: 38px;"></div>
        <div style="float: right; padding-right: 12px; padding-top: 4px;">
          Path <span id="item-path"></span><button class="button-small pure-button button-warning" onclick="window.external.invoke('open')">Change Folder</button> / Total <span class="item-count"></span> / Current Page <span class="item-count"></span> / Marked <span class="item-count"></span> / UnMarked <span class="item-count"></span>
        </div>
      </div>
    </div>

    <router-view></router-view>

    <sidebar-component></sidebar-component>

  </div>

</template>
<script>
import SidebarComponent from "./components/Sidebar.vue";
export default {
  name: "dashboard-page",
  components: {
    SidebarComponent,
  },
  data() {
    return {
    }
  }
};
</script>
