#ifndef OBOE_EXT_H
#define OBOE_EXT_H

#include "oboe/Oboe.h"

namespace oboe {
  typedef DataCallbackResult (*AudioReadyHandler)(void *context,
                                                  AudioStream *oboeStream,
                                                  void *audioData,
                                                  int32_t numFrames);

  typedef void (*ErrorCloseHandler)(void *context,
                                    AudioStream *oboeStream,
                                    Result error);

  class AudioStreamCallbackWrapper: public AudioStreamCallback {
  public:
    AudioStreamCallbackWrapper(const AudioReadyHandler audio_ready,
                               const ErrorCloseHandler before_close,
                               const ErrorCloseHandler after_close);

    void setContext(void *context);

    DataCallbackResult onAudioReady(AudioStream *oboeStream,
                                    void *audioData,
                                    int32_t numFrames);

    void onErrorBeforeClose(AudioStream *oboeStream,
                            Result error);

    void onErrorAfterClose(AudioStream *oboeStream,
                           Result error);

  private:
    void *_context;
    const AudioReadyHandler _audio_ready;
    const ErrorCloseHandler _before_close;
    const ErrorCloseHandler _after_close;
  };

  /*void AudioStreamCallbackWrapper_init(AudioStreamCallbackWrapper *callback,
                                       const AudioReadyHandler audio_ready,
                                       const ErrorCloseHandler before_close,
                                       const ErrorCloseHandler after_close);
  void AudioStreamCallbackWrapper_drop(AudioStreamCallbackWrapper *callback);*/

  AudioStreamCallbackWrapper *
  AudioStreamCallbackWrapper_new(const AudioReadyHandler audio_ready,
                                 const ErrorCloseHandler before_close,
                                 const ErrorCloseHandler after_close);
  void AudioStreamCallbackWrapper_delete(AudioStreamCallbackWrapper *callback);

  //void AudioStreamBuilder_init(AudioStreamBuilder *builder);
  //void AudioStreamBuilder_drop(AudioStreamBuilder *builder);
  AudioStreamBuilder *AudioStreamBuilder_new();
  void AudioStreamBuilder_delete(AudioStreamBuilder *builder);
  void AudioStreamBuilder_setCallback(AudioStreamBuilder *builder,
                                      AudioStreamCallbackWrapper *callback);
  AudioApi AudioStreamBuilder_getAudioApi(const AudioStreamBuilder *builder);
  void AudioStreamBuilder_setAudioApi(AudioStreamBuilder *builder, AudioApi api);
  AudioStreamBase* AudioStreamBuilder_getBase(AudioStreamBuilder *builder);

  void AudioStream_delete(AudioStream *oboeStream);
  Result AudioStream_open(AudioStream *oboeStream);
  Result AudioStream_requestStart(AudioStream *oboeStream);
  Result AudioStream_requestPause(AudioStream *oboeStream);
  Result AudioStream_requestFlush(AudioStream *oboeStream);
  Result AudioStream_requestStop(AudioStream *oboeStream);
  StreamState AudioStream_getState(const AudioStream *oboeStream);
  Result AudioStream_waitForStateChange(AudioStream *oboeStream,
                                        StreamState inputState,
                                        StreamState *nextState,
                                        int64_t timeoutNanoseconds);
  ResultWithValue<int32_t>
  AudioStream_setBufferSizeInFrames(AudioStream *oboeStream,
                                    int32_t requestedFrames);
  ResultWithValue<int32_t>
  AudioStream_getXRunCount(const AudioStream *oboeStream);
  bool AudioStream_isXRunCountSupported(const AudioStream *oboeStream);
  int32_t AudioStream_getFramesPerBurst(AudioStream *oboeStream);
  ResultWithValue<double>
  AudioStream_calculateLatencyMillis(AudioStream *oboeStream);
  AudioApi AudioStream_getAudioApi(const AudioStream *oboeStream);
  ResultWithValue<int32_t> AudioStream_read(AudioStream *oboeStream,
                                            void* buffer,
                                            int32_t numFrames,
                                            int64_t timeoutNanoseconds);
  ResultWithValue<int32_t> AudioStream_write(AudioStream *oboeStream,
                                             const void* buffer,
                                             int32_t numFrames,
                                             int64_t timeoutNanoseconds);

  AudioStreamBase* AudioStream_getBase(AudioStream *oboeStream);
}

#endif
