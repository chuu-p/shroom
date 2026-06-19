package dev.chuu.shroomapp

import androidx.test.ext.junit.runners.AndroidJUnit4
import io.grpc.ManagedChannelBuilder
import kotlinx.coroutines.runBlocking
import org.junit.After
import org.junit.Assert.assertTrue
import org.junit.Assert.fail
import org.junit.Before
import org.junit.Test
import org.junit.runner.RunWith
import shroom.ShroomGrpcKt
import shroom.ShroomService.HealthCheckRequest

@RunWith(AndroidJUnit4::class)
class HealthCheckTest {

    private lateinit var channel: io.grpc.ManagedChannel
    private lateinit var stub: ShroomGrpcKt.ShroomCoroutineStub

    @Before
    fun setUp() {
        channel = ManagedChannelBuilder
            .forAddress("10.0.2.2", 7102)
            .usePlaintext()
            .build()
        stub = ShroomGrpcKt.ShroomCoroutineStub(channel)
    }

    @After
    fun tearDown() {
        channel.shutdown()
    }

    @Test
    fun healthCheck_returnsHealthy() = runBlocking {
        val response = stub.healthCheck(HealthCheckRequest.getDefaultInstance())
        assertTrue("Expected healthy=true from server", response.healthy)
    }
}
