#include "oboe/OboeExt.h"

namespace oboe {
  /*void AudioStreamBuilder_init(AudioStreamBuilder *builder) {
    new (builder) AudioStreamBuilder();
  }

  void AudioStreamBuilder_drop(AudioStreamBuilder *builder) {
    builder->~AudioStreamBuilder();
  }*/

  AudioStreamBuilder *AudioStreamBuilder_new() {
    return new AudioStreamBuilder();
  }

  void AudioStreamBuilder_delete(AudioStreamBuilder *builder) {
    delete builder;
  }

  AudioApi AudioStreamBuilder_getAudioApi(const AudioStreamBuilder *builder) {
    return builder->getAudioApi();
  }

  void AudioStreamBuilder_setAudioApi(AudioStreamBuilder *builder, AudioApi api) {
    builder->setAudioApi(api);
  }

  /// Returns a pointer to a shared_ptr<AudioStreamCallbackWrapper> that must be deleted with
  /// AudioStreamCallbackWrapper_delete.
  void* AudioStreamBuilder_setCallback(AudioStreamBuilder *builder,
                                      void *context,
                                      const DropContextHandler drop_context,
                                      const AudioReadyHandler audio_ready,
                                      const ErrorCloseHandler before_close,
                                      const ErrorCloseHandler after_close) {
    auto *s = new std::shared_ptr<AudioStreamCallbackWrapper>(
            new AudioStreamCallbackWrapper(
                context,
                drop_context,
                audio_ready,
                before_close,
                after_close));
    builder->setDataCallback(*s);
    builder->setErrorCallback(*s);

    return (void*) s;
  }

  AudioStreamBase* AudioStreamBuilder_getBase(AudioStreamBuilder *builder) {
    return static_cast<AudioStreamBase*>(builder);
  }

  Result AudioStreamBuilder_openStreamShared(AudioStreamBuilder *builder,
                                             AudioStream **stream,
                                             void **shared_ptr) {
    std::shared_ptr<AudioStream> *s = new std::shared_ptr<AudioStream>();
    Result res = builder->openStream(*s);
    *stream = s->get();
    *shared_ptr = (void *)s;
    return res;
  }
}
