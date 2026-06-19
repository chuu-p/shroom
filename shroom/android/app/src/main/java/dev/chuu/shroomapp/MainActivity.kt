package dev.chuu.shroomapp

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Button
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import dev.chuu.shroomapp.ui.theme.ShroomAppTheme
import io.grpc.ManagedChannelBuilder
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext
import shroom.ShroomService.HealthCheckRequest
import shroom.ShroomGrpcKt

private const val SERVER_ADDRESS = "0.0.0.0"
private const val SERVER_PORT = 7102

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        enableEdgeToEdge()
        setContent {
            ShroomAppTheme {
                Scaffold(modifier = Modifier.fillMaxSize()) { innerPadding ->
                    HealthCheck(modifier = Modifier.padding(innerPadding))
                }
            }
        }
    }
}

@Composable
fun HealthCheck(modifier: Modifier = Modifier) {
    var status by remember { mutableStateOf("Not checked yet") }
    val scope = rememberCoroutineScope()

    Column(
        modifier = modifier.fillMaxSize().padding(24.dp),
        verticalArrangement = Arrangement.Center,
        horizontalAlignment = Alignment.CenterHorizontally
    ) {
        Button(onClick = {
            scope.launch {
                status = "Checking..."
                status = withContext(Dispatchers.IO) {
                    try {
                        val channel = ManagedChannelBuilder
                            .forAddress(SERVER_ADDRESS, SERVER_PORT)
                            .usePlaintext()
                            .build()
                        val stub = ShroomGrpcKt.ShroomCoroutineStub(channel)
                        try {
                            val response = stub.healthCheck(HealthCheckRequest.getDefaultInstance())
                            "Healthy: ${response.healthy}"
                        } finally {
                            channel.shutdown()
                        }
                    } catch (e: Exception) {
                        "Error: ${e.message} ${e.stackTrace}"
                    }
                }
            }
        }) {
            Text("Health Check")
        }

        Text(
            text = status,
            modifier = Modifier.padding(top = 16.dp)
        )
    }
}

@Preview(showBackground = true)
@Composable
fun HealthCheckPreview() {
    ShroomAppTheme {
        HealthCheck()
    }
}
