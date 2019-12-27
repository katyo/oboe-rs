#include "oboe/OboeExt.h"

namespace oboe {
  Result AudioStream_open(AudioStream *oboeStream) {
    return oboeStream->open();
  }

  Result AudioStream_requestStart(AudioStream *oboeStream) {
    return oboeStream->requestStart();
  }

  Result AudioStream_requestPause(AudioStream *oboeStream) {
    return oboeStream->requestPause();
  }

  Result AudioStream_requestFlush(AudioStream *oboeStream) {
    return oboeStream->requestFlush();
  }

  Result AudioStream_requestStop(AudioStream *oboeStream) {
    return oboeStream->requestStop();
  }

  StreamState AudioStream_getState(const AudioStream *oboeStream) {
    return oboeStream->getState();
  }

  Result AudioStream_waitForStateChange(AudioStream *oboeStream,
                                        StreamState inputState,
                                        StreamState *nextState,
                                        int64_t timeoutNanoseconds) {
    return oboeStream->waitForStateChange(inputState,
                                          nextState,
                                          timeoutNanoseconds);
  }

  ResultWithValue<int32_t>
  AudioStream_setBufferSizeInFrames(AudioStream *oboeStream,
                                    int32_t requestedFrames) {
    return oboeStream->setBufferSizeInFrames(requestedFrames);
  }

  ResultWithValue<int32_t>
  AudioStream_getXRunCount(const AudioStream *oboeStream) {
    return oboeStream->getXRunCount();
  }

  bool AudioStream_isXRunCountSupported(const AudioStream *oboeStream) {
    return oboeStream->isXRunCountSupported();
  }

  int32_t AudioStream_getFramesPerBurst(AudioStream *oboeStream) {
    return oboeStream->getFramesPerBurst();
  }

  ResultWithValue<double>
  AudioStream_calculateLatencyMillis(AudioStream *oboeStream) {
    return oboeStream->calculateLatencyMillis();
  }

  AudioApi AudioStream_getAudioApi(const AudioStream *oboeStream) {
    return oboeStream->getAudioApi();
  }
}
