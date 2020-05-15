import UIKit from 'uikit'
import Icons from 'uikit/dist/js/uikit-icons'
import { init as initRepos } from './githubRepos.js'
import { init as initSetupForm } from './setupForm.js'
import 'uikit/dist/css/uikit.css'
import '../styles/styles.css'

UIKit.use(Icons)

window.onload = () => {
    initRepos()
    initSetupForm()
}
