import Vue from 'vue';
import App from './App.vue';
import VueCompositionApi from '@vue/composition-api'
import Vuetify from 'vuetify'

Vue.config.productionTip = false;
Vue.config.devtools = false;

Vue.use(VueCompositionApi);
Vue.use(Vuetify);

const vuetify = new Vuetify({
    theme: {
      
    },
    icons: {
      iconfont: 'mdi'
    }
})

new Vue({
    render: createElement => createElement(App),
    vuetify,
}).$mount('#app')