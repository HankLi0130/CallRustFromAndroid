package hankdev.app.android

import android.os.Bundle
import android.util.Log
import androidx.appcompat.app.AppCompatActivity
import hankdev.app.android.databinding.ActivityMainBinding

class MainActivity : AppCompatActivity() {
    private val tag = "MainActivity"
    private lateinit var binding: ActivityMainBinding

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        binding = ActivityMainBinding.inflate(layoutInflater)
        setContentView(binding.root)

        with(binding) {
            call1.setOnClickListener { Log.i(tag, RustLib.hello("Android")) }
            call2.setOnClickListener { RustLib.initLogging() }
            call3.setOnClickListener { RustLib.showLog() }
        }
    }
}