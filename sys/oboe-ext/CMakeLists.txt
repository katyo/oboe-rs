cmake_minimum_required(VERSION 3.4.1)

project(oboe-ext)

set(oboe-ext_sources
    src/AudioStreamWrapper.cpp
    src/AudioStreamBuilderWrapper.cpp
    src/AudioStreamCallbackWrapper.cpp
    )

set(oboe_sources
    src/aaudio/AAudioLoader.cpp
    src/aaudio/AudioStreamAAudio.cpp
    src/common/AudioSourceCaller.cpp
    src/common/AudioStream.cpp
    src/common/AudioStreamBuilder.cpp
    src/common/DataConversionFlowGraph.cpp
    src/common/FilterAudioStream.cpp
    src/common/FixedBlockAdapter.cpp
    src/common/FixedBlockReader.cpp
    src/common/FixedBlockWriter.cpp
    src/common/LatencyTuner.cpp
    src/common/SourceFloatCaller.cpp
    src/common/SourceI16Caller.cpp
    src/common/Utilities.cpp
    src/common/QuirksManager.cpp
    src/fifo/FifoBuffer.cpp
    src/fifo/FifoController.cpp
    src/fifo/FifoControllerBase.cpp
    src/fifo/FifoControllerIndirect.cpp
    src/flowgraph/FlowGraphNode.cpp
    src/flowgraph/ChannelCountConverter.cpp
    src/flowgraph/ClipToRange.cpp
    src/flowgraph/ManyToMultiConverter.cpp
    src/flowgraph/MonoToMultiConverter.cpp
    src/flowgraph/MultiToMonoConverter.cpp
    src/flowgraph/RampLinear.cpp
    src/flowgraph/SampleRateConverter.cpp
    src/flowgraph/SinkFloat.cpp
    src/flowgraph/SinkI16.cpp
    src/flowgraph/SinkI24.cpp
    src/flowgraph/SourceFloat.cpp
    src/flowgraph/SourceI16.cpp
    src/flowgraph/SourceI24.cpp
    src/flowgraph/resampler/IntegerRatio.cpp
    src/flowgraph/resampler/LinearResampler.cpp
    src/flowgraph/resampler/MultiChannelResampler.cpp
    src/flowgraph/resampler/PolyphaseResampler.cpp
    src/flowgraph/resampler/PolyphaseResamplerMono.cpp
    src/flowgraph/resampler/PolyphaseResamplerStereo.cpp
    src/flowgraph/resampler/SincResampler.cpp
    src/flowgraph/resampler/SincResamplerStereo.cpp
    src/opensles/AudioInputStreamOpenSLES.cpp
    src/opensles/AudioOutputStreamOpenSLES.cpp
    src/opensles/AudioStreamBuffered.cpp
    src/opensles/AudioStreamOpenSLES.cpp
    src/opensles/EngineOpenSLES.cpp
    src/opensles/OpenSLESUtilities.cpp
    src/opensles/OutputMixerOpenSLES.cpp
    src/common/StabilizedCallback.cpp
    src/common/Trace.cpp
    src/common/Version.cpp
    )

list(TRANSFORM oboe_sources PREPEND ${OBOE_DIR}/)

add_library(oboe-ext ${oboe-ext_sources} ${oboe_sources})

target_include_directories(oboe-ext
    PRIVATE src ${OBOE_DIR}/src
    PUBLIC include ${OBOE_DIR}/include)

target_compile_options(oboe-ext PRIVATE
    -std=c++14
    -Wall
    -Wextra-semi
    -Wshadow
    -Wshadow-field
    -fno-rtti
    -fno-exceptions
    -Ofast
    "$<$<CONFIG:Debug>:-Werror>")

target_compile_definitions(oboe-ext PUBLIC
    $<$<CONFIG:DEBUG>:OBOE_ENABLE_LOGGING=1>)
