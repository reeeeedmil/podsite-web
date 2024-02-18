from django.urls import path
from subNet import views

urlpatterns = [
    path('net/<int:pk>', views.NetViewSet.as_view({'get': 'retrieve'})),
    path('prefixes/', views.PrefixPost.as_view({'post': 'create'})),
]
