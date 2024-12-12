package hankdev.app.android

import androidx.appcompat.app.AppCompatActivity
import android.os.Bundle
import android.util.Log
import hankdev.app.android.databinding.ActivityMainBinding

class MainActivity : AppCompatActivity() {
    private val tag = "MainActivity"
    private lateinit var binding: ActivityMainBinding

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        binding = ActivityMainBinding.inflate(layoutInflater)
        setContentView(binding.root)

        binding.call1.setOnClickListener { Log.i(tag, RustLib.hello("Android")) }
    }
}