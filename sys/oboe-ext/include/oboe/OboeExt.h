#ifndef OBOE_EXT_H
#define OBOE_EXT_H

#include "oboe/Oboe.h"

namespace oboe {
  typedef void (*DropContextHandler)(void *context);

  typedef DataCallbackResult (*AudioReadyHandler)(void *context,
                                                  AudioStream *oboeStream,
                                                  void *audioData,
                                                  int32_t numFrames);

  typedef void (*ErrorCloseHandler)(void *context,
                                    AudioStream *oboeStream,
                                    Result error);

  class AudioStreamCallbackWrapper
      : public AudioStreamDataCallback, public AudioStreamErrorCallback {
  public:
    AudioStreamCallbackWrapper(void *context,
                               const DropContextHandler drop_context,
                               const AudioReadyHandler audio_ready,
                               const ErrorCloseHandler before_close,
                               const ErrorCloseHandler after_close);

    ~AudioStreamCallbackWrapper();

    DataCallbackResult onAudioReady(AudioStream *oboeStream,
                                    void *audioData,
                                    int32_t numFrames);

    void onErrorBeforeClose(AudioStream *oboeStream,
                            Result error);

    void onErrorAfterClose(AudioStream *oboeStream,
                           Result error);

  private:
    void *_context;
    const DropContextHandler _drop_context;
    const AudioReadyHandler _audio_ready;
    const ErrorCloseHandler _before_close;
    const ErrorCloseHandler _after_close;
  };

  AudioStreamBuilder *AudioStreamBuilder_new();
  void AudioStreamBuilder_delete(AudioStreamBuilder *builder);
  void AudioStreamBuilder_setCallback(AudioStreamBuilder *builder,
                                      void *context,
                                      const DropContextHandler drop_context,
                                      const AudioReadyHandler audio_ready,
                                      const ErrorCloseHandler before_close,
                                      const ErrorCloseHandler after_close);

  AudioApi AudioStreamBuilder_getAudioApi(const AudioStreamBuilder *builder);
  void AudioStreamBuilder_setAudioApi(AudioStreamBuilder *builder, AudioApi api);
  AudioStreamBase* AudioStreamBuilder_getBase(AudioStreamBuilder *builder);
  Result AudioStreamBuilder_openStreamShared(AudioStreamBuilder *builder,
                                             AudioStream **stream,
                                             void **shared_ptr);
  
  void AudioStream_delete(AudioStream *oboeStream);
  void AudioStream_deleteShared(void *shared_ptr);
  Result AudioStream_open(AudioStream *oboeStream);
  Result AudioStream_close(AudioStream *oboeStream);
  Result AudioStream_requestStart(AudioStream *oboeStream);
  Result AudioStream_requestPause(AudioStream *oboeStream);
  Result AudioStream_requestFlush(AudioStream *oboeStream);
  Result AudioStream_requestStop(AudioStream *oboeStream);
  StreamState AudioStream_getState(AudioStream *oboeStream);
  Result AudioStream_waitForStateChange(AudioStream *oboeStream,
                                        StreamState inputState,
                                        StreamState *nextState,
                                        int64_t timeoutNanoseconds);
  ResultWithValue<int32_t>
  AudioStream_setBufferSizeInFrames(AudioStream *oboeStream,
                                    int32_t requestedFrames);
  ResultWithValue<int32_t>
  AudioStream_getXRunCount(AudioStream *oboeStream);
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
