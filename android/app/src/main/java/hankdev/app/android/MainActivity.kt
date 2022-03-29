package hankdev.app.android

import androidx.appcompat.app.AppCompatActivity
import android.os.Bundle
import android.widget.TextView

class MainActivity : AppCompatActivity(), JNICallback {

    init {
        System.loadLibrary("rust_android")
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)

        invokeCallbackViaJNI(this)
    }

    /**
     * A native method that is implemented by the 'rust' native library,
     * which is packaged with this application.
     */
    external fun invokeCallbackViaJNI(callback: JNICallback)

    override fun callback(string: String) {
        findViewById<TextView>(R.id.helloLabel).text = string
    }
}