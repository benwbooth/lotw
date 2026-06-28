// Minimal shim to set the application/window icon from raw RGBA pixels.
// cxx-qt-lib does not wrap QIcon/QPixmap, so we expose a tiny C entry point and
// build a QIcon here. The pixel data is copied (QImage::copy) so the caller's
// buffer need not outlive the call.
#include <QtGui/QGuiApplication>
#include <QtGui/QIcon>
#include <QtGui/QImage>
#include <QtGui/QPixmap>

extern "C" void lotw_set_window_icon_rgba(const unsigned char *data, int w, int h) {
    if (!data || w <= 0 || h <= 0) {
        return;
    }
    QImage img(data, w, h, w * 4, QImage::Format_RGBA8888);
    QGuiApplication::setWindowIcon(QIcon(QPixmap::fromImage(img.copy())));
}
