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

  void AudioStreamBuilder_setCallback(AudioStreamBuilder *builder,
                                      AudioStreamCallbackWrapper *callback) {
    builder->setCallback(callback);
  }
}
